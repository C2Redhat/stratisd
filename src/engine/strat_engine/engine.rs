// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::path::Path;
use std::collections::BTreeMap;
use std::collections::btree_map::Entry;
use std::collections::BTreeSet;
use std::str::FromStr;
use std::iter::FromIterator;

use devicemapper::Device;

use engine::Engine;
use engine::EngineError;
use engine::EngineResult;
use engine::ErrorEnum;
use engine::Pool;

use super::pool::StratPool;


#[derive(Debug)]
pub struct StratEngine {
    pub pools: BTreeMap<String, StratPool>,
}

impl StratEngine {
    pub fn new() -> StratEngine {
        StratEngine { pools: BTreeMap::new() }
    }
}

impl Engine for StratEngine {
    fn configure_simulator(&mut self, _denominator: u32) -> EngineResult<()> {
        Ok(()) // we're not the simulator and not configurable, so just say ok
    }

    fn create_pool(&mut self,
                   name: &str,
                   blockdev_paths: &[&Path],
                   raid_level: u16,
                   force: bool)
                   -> EngineResult<usize> {

        if self.pools.contains_key(name) {
            return Err(EngineError::Stratis(ErrorEnum::AlreadyExists(name.into())));
        }

        let mut devices = BTreeSet::new();
        for path in blockdev_paths {
            let dev = try!(Device::from_str(&path.to_string_lossy()));
            devices.insert(dev);
        }

        let pool = try!(StratPool::new(name, devices, raid_level, force));
        let num_bdevs = pool.block_devs.len();

        self.pools.insert(name.to_owned(), pool);
        Ok(num_bdevs)
    }

    /// Destroy a pool, if the pool does not exist, return Ok.
    fn destroy_pool(&mut self, name: &str) -> EngineResult<bool> {
        destroy_pool!{self; name}
    }

    fn get_pool(&mut self, name: &str) -> EngineResult<&mut Pool> {
        get_pool!(self; name)
    }

    fn pools(&mut self) -> BTreeMap<&str, &mut Pool> {
        pools!(self)
    }
}
