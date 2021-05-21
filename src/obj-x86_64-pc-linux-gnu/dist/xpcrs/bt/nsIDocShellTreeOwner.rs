//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/docshell/base/nsIDocShellTreeOwner.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDocShellTreeOwner",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void contentShellAdded (in nsIDocShellTreeItem aContentShell, in boolean aPrimary); */
                    Method {
                        name: "ContentShellAdded",
                        params: &[Param { name: "aContentShell", ty: "*const nsIDocShellTreeItem" }, Param { name: "aPrimary", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void contentShellRemoved (in nsIDocShellTreeItem aContentShell); */
                    Method {
                        name: "ContentShellRemoved",
                        params: &[Param { name: "aContentShell", ty: "*const nsIDocShellTreeItem" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIDocShellTreeItem primaryContentShell; */
                    Method {
                        name: "GetPrimaryContentShell",
                        params: &[Param { name: "aPrimaryContentShell", ty: "*mut*const nsIDocShellTreeItem" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void remoteTabAdded (in nsIRemoteTab aTab, in boolean aPrimary); */
                    Method {
                        name: "RemoteTabAdded",
                        params: &[Param { name: "aTab", ty: "*const nsIRemoteTab" }, Param { name: "aPrimary", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void remoteTabRemoved (in nsIRemoteTab aTab); */
                    Method {
                        name: "RemoteTabRemoved",
                        params: &[Param { name: "aTab", ty: "*const nsIRemoteTab" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIRemoteTab primaryRemoteTab; */
                    Method {
                        name: "GetPrimaryRemoteTab",
                        params: &[Param { name: "aPrimaryRemoteTab", ty: "*mut*const nsIRemoteTab" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] void sizeShellTo (in nsIDocShellTreeItem shell, in long cx, in long cy); */
                    Method {
                        name: "SizeShellTo",
                        params: &[Param { name: "shell", ty: "*const nsIDocShellTreeItem" }, Param { name: "cx", ty: "i32" }, Param { name: "cy", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void getPrimaryContentSize (out long width, out long height); */
                    Method {
                        name: "GetPrimaryContentSize",
                        params: &[Param { name: "width", ty: "*mut i32" }, Param { name: "height", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setPrimaryContentSize (in long width, in long height); */
                    Method {
                        name: "SetPrimaryContentSize",
                        params: &[Param { name: "width", ty: "i32" }, Param { name: "height", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void getRootShellSize (out long width, out long height); */
                    Method {
                        name: "GetRootShellSize",
                        params: &[Param { name: "width", ty: "*mut i32" }, Param { name: "height", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setRootShellSize (in long width, in long height); */
                    Method {
                        name: "SetRootShellSize",
                        params: &[Param { name: "width", ty: "i32" }, Param { name: "height", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setPersistence (in boolean aPersistPosition, in boolean aPersistSize, in boolean aPersistSizeMode); */
                    Method {
                        name: "SetPersistence",
                        params: &[Param { name: "aPersistPosition", ty: "bool" }, Param { name: "aPersistSize", ty: "bool" }, Param { name: "aPersistSizeMode", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void getPersistence (out boolean aPersistPosition, out boolean aPersistSize, out boolean aPersistSizeMode); */
                    Method {
                        name: "GetPersistence",
                        params: &[Param { name: "aPersistPosition", ty: "*mut bool" }, Param { name: "aPersistSize", ty: "*mut bool" }, Param { name: "aPersistSizeMode", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long tabCount; */
                    Method {
                        name: "GetTabCount",
                        params: &[Param { name: "aTabCount", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute bool hasPrimaryContent; */
                    Method {
                        name: "GetHasPrimaryContent",
                        params: &[Param { name: "aHasPrimaryContent", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

