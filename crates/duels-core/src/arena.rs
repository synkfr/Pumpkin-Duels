use duels_storage::{ArenaRecord, BoundsDto, PositionDto};
use pumpkin_plugin_api::server::Dimension;
use pumpkin_plugin_api::Server;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub struct ArenaLocation {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
}

impl From<PositionDto> for ArenaLocation {
    fn from(dto: PositionDto) -> Self {
        Self {
            x: dto.x,
            y: dto.y,
            z: dto.z,
            yaw: dto.yaw,
            pitch: dto.pitch,
        }
    }
}

impl From<ArenaLocation> for PositionDto {
    fn from(loc: ArenaLocation) -> Self {
        Self {
            x: loc.x,
            y: loc.y,
            z: loc.z,
            yaw: loc.yaw,
            pitch: loc.pitch,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ArenaBounds {
    pub min_x: i32,
    pub max_x: i32,
    pub min_z: i32,
    pub max_z: i32,
    pub min_y: i32,
    pub max_y: i32,
    pub ringout_y: Option<f64>,
}

impl From<BoundsDto> for ArenaBounds {
    fn from(dto: BoundsDto) -> Self {
        Self {
            min_x: dto.min_x,
            max_x: dto.max_x,
            min_z: dto.min_z,
            max_z: dto.max_z,
            min_y: dto.min_y,
            max_y: dto.max_y,
            ringout_y: dto.ringout_y,
        }
    }
}

impl From<ArenaBounds> for BoundsDto {
    fn from(b: ArenaBounds) -> Self {
        Self {
            min_x: b.min_x,
            max_x: b.max_x,
            min_z: b.min_z,
            max_z: b.max_z,
            min_y: b.min_y,
            max_y: b.max_y,
            ringout_y: b.ringout_y,
        }
    }
}

impl ArenaBounds {
    pub fn is_inside(&self, x: i32, y: i32, z: i32) -> bool {
        x >= self.min_x
            && x <= self.max_x
            && z >= self.min_z
            && z <= self.max_z
            && y >= self.min_y
            && y <= self.max_y
    }

    pub fn is_ringout(&self, y: f64) -> bool {
        if let Some(limit) = self.ringout_y {
            y < limit
        } else {
            false
        }
    }
}

#[derive(Clone, Debug)]
pub struct ArenaTemplate {
    pub name: String,
    pub template_world: String,
    pub spawn1: ArenaLocation,
    pub spawn2: ArenaLocation,
    pub spectator_spawn: ArenaLocation,
    pub bounds: ArenaBounds,
    pub enabled: bool,
}

impl From<ArenaRecord> for ArenaTemplate {
    fn from(record: ArenaRecord) -> Self {
        Self {
            name: record.name,
            template_world: record.template_world,
            spawn1: record.spawn1.into(),
            spawn2: record.spawn2.into(),
            spectator_spawn: record.spectator_spawn.into(),
            bounds: record.bounds.into(),
            enabled: record.enabled,
        }
    }
}

impl From<ArenaTemplate> for ArenaRecord {
    fn from(template: ArenaTemplate) -> Self {
        Self {
            name: template.name,
            template_world: template.template_world,
            spawn1: template.spawn1.into(),
            spawn2: template.spawn2.into(),
            spectator_spawn: template.spectator_spawn.into(),
            bounds: template.bounds.into(),
            enabled: template.enabled,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ActiveArenaInstance {
    pub instance_id: String,
    pub world_name: String,
    pub template_name: String,
    pub spawn1: ArenaLocation,
    pub spawn2: ArenaLocation,
    pub spectator_spawn: ArenaLocation,
    pub bounds: ArenaBounds,
}

pub struct ArenaInstanceManager {
    worlds_dir: PathBuf,
}

impl ArenaInstanceManager {
    pub fn new<P: AsRef<Path>>(worlds_dir: P) -> Self {
        Self {
            worlds_dir: worlds_dir.as_ref().to_path_buf(),
        }
    }

    fn find_world_dir(&self, world_name: &str) -> Option<PathBuf> {
        let candidates = [
            self.worlds_dir.join(world_name),
            PathBuf::from(format!("world/{world_name}")),
            PathBuf::from(world_name),
        ];
        candidates.into_iter().find(|p| p.exists() && p.is_dir())
    }

    fn resolve_world_dir(&self, world_name: &str) -> PathBuf {
        if self.worlds_dir.exists() {
            self.worlds_dir.join(world_name)
        } else {
            PathBuf::from(format!("world/{world_name}"))
        }
    }

    pub fn delete_world_dir(&self, world_name: &str) {
        let candidates = [
            self.worlds_dir.join(world_name),
            PathBuf::from(format!("world/{world_name}")),
            PathBuf::from(world_name),
        ];
        for p in candidates {
            if p.exists() {
                let _ = std::fs::remove_dir_all(p);
            }
        }
    }

    pub fn create_instance(
        &self,
        server: &Server,
        template: &ArenaTemplate,
        match_id: &str,
    ) -> Result<ActiveArenaInstance, String> {
        let instance_world_name = format!("duel_{match_id}");

        let template_dir = self.find_world_dir(&template.template_world);
        let target_dir = self.resolve_world_dir(&instance_world_name);

        if let Some(src) = template_dir
            && src.exists() && src.is_dir()
        {
            let _ = copy_dir_all(&src, &target_dir);
        }

        let _ = server.create_world(&instance_world_name, Dimension::Overworld);

        Ok(ActiveArenaInstance {
            instance_id: match_id.to_string(),
            world_name: instance_world_name,
            template_name: template.name.clone(),
            spawn1: template.spawn1.clone(),
            spawn2: template.spawn2.clone(),
            spectator_spawn: template.spectator_spawn.clone(),
            bounds: template.bounds.clone(),
        })
    }

    pub fn destroy_instance(
        &self,
        server: &Server,
        instance: &ActiveArenaInstance,
    ) -> Result<(), String> {
        let _ = server.unload_world(&instance.world_name);
        self.delete_world_dir(&instance.world_name);
        Ok(())
    }
}

fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let ft = entry.file_type()?;
        let from = entry.path();
        let to = dst.join(entry.file_name());
        if ft.is_dir() {
            copy_dir_all(&from, &to)?;
        } else {
            let _ = std::fs::copy(&from, &to);
        }
    }
    Ok(())
}
