//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/passwordmgr/nsILoginMetaInfo.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsILoginMetaInfo",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* attribute AString guid; */
                    Method {
                        name: "GetGuid",
                        params: &[Param { name: "aGuid", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetGuid",
                        params: &[Param { name: "aGuid", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute unsigned long long timeCreated; */
                    Method {
                        name: "GetTimeCreated",
                        params: &[Param { name: "aTimeCreated", ty: "*mut u64" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetTimeCreated",
                        params: &[Param { name: "aTimeCreated", ty: "u64" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute unsigned long long timeLastUsed; */
                    Method {
                        name: "GetTimeLastUsed",
                        params: &[Param { name: "aTimeLastUsed", ty: "*mut u64" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetTimeLastUsed",
                        params: &[Param { name: "aTimeLastUsed", ty: "u64" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute unsigned long long timePasswordChanged; */
                    Method {
                        name: "GetTimePasswordChanged",
                        params: &[Param { name: "aTimePasswordChanged", ty: "*mut u64" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetTimePasswordChanged",
                        params: &[Param { name: "aTimePasswordChanged", ty: "u64" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute unsigned long timesUsed; */
                    Method {
                        name: "GetTimesUsed",
                        params: &[Param { name: "aTimesUsed", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetTimesUsed",
                        params: &[Param { name: "aTimesUsed", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

