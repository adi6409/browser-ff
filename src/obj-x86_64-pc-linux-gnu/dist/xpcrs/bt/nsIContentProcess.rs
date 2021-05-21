//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/base/nsIContentProcess.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIContentProcessInfo",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute boolean isAlive; */
                    Method {
                        name: "GetIsAlive",
                        params: &[Param { name: "aIsAlive", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute int32_t processId; */
                    Method {
                        name: "GetProcessId",
                        params: &[Param { name: "aProcessId", ty: "*mut int32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute int32_t tabCount; */
                    Method {
                        name: "GetTabCount",
                        params: &[Param { name: "aTabCount", ty: "*mut int32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsISupports messageManager; */
                    Method {
                        name: "GetMessageManager",
                        params: &[Param { name: "aMessageManager", ty: "*mut *const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIContentProcessProvider",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* int32_t provideProcess (in AUTF8String aType, in Array<nsIContentProcessInfo> aAliveProcesses, in uint32_t aMaxCount); */
                    Method {
                        name: "ProvideProcess",
                        params: &[Param { name: "aType", ty: "*const ::nsstring::nsACString" }, Param { name: "aAliveProcesses", ty: "*const thin_vec::ThinVec<RefPtr<nsIContentProcessInfo>>" }, Param { name: "aMaxCount", ty: "uint32_t" }, Param { name: "_retval", ty: "*mut int32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

