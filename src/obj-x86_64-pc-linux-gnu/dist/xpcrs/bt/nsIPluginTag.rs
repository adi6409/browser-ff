//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/plugins/base/nsIPluginTag.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPluginTag",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute AUTF8String description; */
                    Method {
                        name: "GetDescription",
                        params: &[Param { name: "aDescription", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AUTF8String filename; */
                    Method {
                        name: "GetFilename",
                        params: &[Param { name: "aFilename", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AUTF8String fullpath; */
                    Method {
                        name: "GetFullpath",
                        params: &[Param { name: "aFullpath", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AUTF8String version; */
                    Method {
                        name: "GetVersion",
                        params: &[Param { name: "aVersion", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AUTF8String name; */
                    Method {
                        name: "GetName",
                        params: &[Param { name: "aName", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AUTF8String niceName; */
                    Method {
                        name: "GetNiceName",
                        params: &[Param { name: "aNiceName", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [infallible] readonly attribute boolean blocklisted; */
                    Method {
                        name: "GetBlocklisted",
                        params: &[Param { name: "aBlocklisted", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [infallible] readonly attribute boolean isEnabledStateLocked; */
                    Method {
                        name: "GetIsEnabledStateLocked",
                        params: &[Param { name: "aIsEnabledStateLocked", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [infallible] readonly attribute boolean active; */
                    Method {
                        name: "GetActive",
                        params: &[Param { name: "aActive", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [infallible] readonly attribute unsigned long blocklistState; */
                    Method {
                        name: "GetBlocklistState",
                        params: &[Param { name: "aBlocklistState", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [infallible] readonly attribute boolean disabled; */
                    Method {
                        name: "GetDisabled",
                        params: &[Param { name: "aDisabled", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [infallible] readonly attribute boolean clicktoplay; */
                    Method {
                        name: "GetClicktoplay",
                        params: &[Param { name: "aClicktoplay", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [infallible] readonly attribute boolean loaded; */
                    Method {
                        name: "GetLoaded",
                        params: &[Param { name: "aLoaded", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute unsigned long enabledState; */
                    Method {
                        name: "GetEnabledState",
                        params: &[Param { name: "aEnabledState", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetEnabledState",
                        params: &[Param { name: "aEnabledState", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute PRTime lastModifiedTime; */
                    Method {
                        name: "GetLastModifiedTime",
                        params: &[Param { name: "aLastModifiedTime", ty: "*mut PRTime" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean isFlashPlugin; */
                    Method {
                        name: "GetIsFlashPlugin",
                        params: &[Param { name: "aIsFlashPlugin", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* Array<AUTF8String> getMimeTypes (); */
                    Method {
                        name: "GetMimeTypes",
                        params: &[Param { name: "_retval", ty: "*mut thin_vec::ThinVec<::nsstring::nsCString>" }],
                        ret: "::nserror::nsresult",
                    },

                    /* Array<AUTF8String> getMimeDescriptions (); */
                    Method {
                        name: "GetMimeDescriptions",
                        params: &[Param { name: "_retval", ty: "*mut thin_vec::ThinVec<::nsstring::nsCString>" }],
                        ret: "::nserror::nsresult",
                    },

                    /* Array<AUTF8String> getExtensions (); */
                    Method {
                        name: "GetExtensions",
                        params: &[Param { name: "_retval", ty: "*mut thin_vec::ThinVec<::nsstring::nsCString>" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long id; */
                    Method {
                        name: "GetId",
                        params: &[Param { name: "aId", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIFakePluginTag",
            base: Some("nsIPluginTag"),
            methods: Ok(&[
                    /* readonly attribute nsIURI handlerURI; */
                    Method {
                        name: "GetHandlerURI",
                        params: &[Param { name: "aHandlerURI", ty: "*mut*const nsIURI" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString sandboxScript; */
                    Method {
                        name: "GetSandboxScript",
                        params: &[Param { name: "aSandboxScript", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

