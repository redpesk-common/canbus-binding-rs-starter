// SPDX-License-Identifier: GPL-3.0-or-later
//
// This file is part of canbus-binding-rs.
//
// canbus-binding-rs is free software: you can redistribute it and/or modify it
// under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// canbus-binding-rs is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use afbv4::prelude::*;
// Import helper that creates verbs/events from a DBC pool.
use dbcapi::create_pool_verbs;
// Import parser for the JSON configuration describing sockcan and API parameters.
use sockdata::types::parse_sockcan_config;

// Include generated DBC message pool for the starter project.
include!("./__starter.rs");
use crate::Starter::*;

/// Binding entry point.
/// Runs when the shared object is loaded; create and register the API here.
///
/// Order matters:
/// 1) Build the API descriptor and declare dependencies
/// 2) **Finalize** to obtain a valid apiv4 handle (non-NULL)
/// 3) Register verbs/events using the finalized handle
/// 4) Return the finalized API handle to libafb
pub fn binding_init(rootv4: AfbApiV4, jconf: JsoncObj) -> Result<&'static AfbApi, AfbError> {
    // Log raw configuration for traceability; be careful with potential sensitive data
    // (ACLs, bus names, credentials) and ensure log level is appropriate.
    afb_log_msg!(Info, rootv4, "config:{}", jconf);

    // Parse and validate JSON configuration into a strongly-typed structure.
    let config = parse_sockcan_config(&jconf);

    // create a new api
    // Create and configure the public API:
    // - set API identifier and human-readable info,
    // - attach permission/ACL information,
    // - keep the API unsealed while verbs/events are being added.
    let can_api = AfbApi::new(config.api_uid)
        .set_info(config.info)
        .set_permission(AfbPermission::new(to_static_str(config.acls.to_owned())))
        .seal(false)
        .require_api(config.sock_api);

    // Instantiate the DBC message pool for the starter CAN network,
    // and register verbs/events for each message/signal defined in the DBC.
    let pool = Box::new(CanMsgPool::new(config.api_uid));

    // Create verbs and events from the DBC pool and register them on the API.
    create_pool_verbs(rootv4, can_api, jconf, pool)?;
    // Finalize the API so it becomes visible/usable by clients.
    // After this call the API descriptor is no longer mutable.
    can_api.finalize()
}

// Register the binding entry point with libafb.
// libafb will call `binding_init` during module load to initialize the API.
// register binding within libafb
AfbBindingRegister!(binding_init);
