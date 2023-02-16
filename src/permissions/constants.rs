// all permissions are in one of these three arrays

pub const VIEWER_PERMISSIONS: &[&str] = &[
    // planet permissions
    "+planet.view",        // view the planet
    "+planet.member.view", // view members
    // page permissions
    "+page.view", // view pages
    // forum permissions
    "+forum.view", // view forums
    // file permissions
    "+files.view",             // view files
    "+files.search",           // search the file tree
    "+files.files.download",   // download files
    "+files.folders.download", // download folders
    // chat permissions
    "+chat.view",   // view chats
    "+chat.search", // search chat messages
];

pub const MEMBER_PERMISSIONS: &[&str] = &[
    // forum permissions
    "+forum.posts.create",       // create a forum post
    "+forum.posts.edit.own",     // edit their own posts
    "+forum.posts.delete.own",   // delete their own posts
    "+forum.posts.react.own",    // react to their own posts
    "+forum.posts.react.others", // react to other's posts
    "+forum.posts.attach",       // attach files to posts
    "+forum.post.reply",         // reply to posts
    "+forum.tags.use",           // use tags in their files
    // chat permissions
    "+chat.messages.send",         // send chat messages
    "+chat.messages.delete.own",   // delete own chat messages
    "+chat.messages.edit.own",     // edit own chat messages
    "+chat.messages.react.own",    // react to own chat messages
    "+chat.messages.react.others", // react to other's chat messages
    "+chat.messages.attach",       // attach files to chat messages
];

pub const ADMINISTRATOR_PERMISSIONS: &[&str] = &[
    // special permissions
    "+administrator",           // group permission
    "+administrator.read_all",  // overrides per-component read rules
    "+administrator.write_all", // overrides per-component write rules
    // planet permissions
    "+planet.change_name",             // change the planet name
    "+planet.change_description",      // change the planet description
    "+planet.component.create",        // create new components
    "+planet.component.rename",        // rename components
    "+planet.component.delete",        // delete components
    "+planet.component.move",          // move components
    "+planet.component.set_home",      // change the home component
    "+planet.roles.create",            // create roles
    "+planet.roles.rename",            // rename roles
    "+planet.roles.change_color",      // change the color of roles
    "+planet.roles.edit",              // change the permissions of roles
    "+planet.roles.delete",            // delete roles
    "+planet.roles.reorder",           // change the order of roles
    "+planet.roles.add_member",        // add new members to roles
    "+planet.member.edit_permissions", // edit a member's permissions
    "+planet.emojis.create",           // create emojis
    "+planet.emojis.delete",           // delete emojis
    "+planet.change_css",              // change the planet's css
    // page permissions
    "+page.edit", // edit pages
    // forum permissions
    "+forum.posts.edit.others",   // edit other's posts
    "+forum.posts.delete.others", // delete other's posts
    "+forum.posts.sticky",        // sticky/unsticky posts
    "+forum.posts.lock.set",      // lock/unlock posts
    "+forum.posts.lock.ignore",   // post replies to unlocked posts
    "+forum.tags.create",         // create tags
    "+forum.tags.delete",         // delete tags
    // files permissions
    "+files.files.upload",          // upload files
    "+files.files.delete.own",      // delete own files
    "+files.files.delete.others",   // delete other's files
    "+files.files.rename.own",      // rename own files
    "+files.files.rename.others",   // rename other's files
    "+files.folders.create",        // create folders
    "+files.folders.delete.own",    // delete own folders
    "+files.folders.delete.others", // delete other's folders
    "+files.folders.rename.own",    // rename own folders
    "+files.folders.rename.others", // rename other's folders
    // chat permissions
    "+chat.set_topic",              // set the chat topic
    "+chat.messages.delete.others", // delete other's messages
    "+chat.messages.edit.others",   // delete other's edits
    "+chat.messages.pin.own",       // pin own messages
    "+chat.messages.pin.others",    // pin other's messages
];

pub const OWNER_PERMISSIONS: &[&str] = &[
    "+owner",                   // group permission
    "+planet.change_publicity", // change whether or not the planet is public
    "+planet.delete",           // delete the planet
];
