//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIProxyInfo.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIProxyInfo",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute AUTF8String host; */
                    Method {
                        name: "GetHost",
                        params: &[Param { name: "aHost", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long port; */
                    Method {
                        name: "GetPort",
                        params: &[Param { name: "aPort", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute ACString type; */
                    Method {
                        name: "GetType",
                        params: &[Param { name: "aType", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long flags; */
                    Method {
                        name: "GetFlags",
                        params: &[Param { name: "aFlags", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute unsigned long resolveFlags; */
                    Method {
                        name: "GetResolveFlags",
                        params: &[Param { name: "aResolveFlags", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetResolveFlags",
                        params: &[Param { name: "aResolveFlags", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute ACString username; */
                    Method {
                        name: "GetUsername",
                        params: &[Param { name: "aUsername", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute ACString password; */
                    Method {
                        name: "GetPassword",
                        params: &[Param { name: "aPassword", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long failoverTimeout; */
                    Method {
                        name: "GetFailoverTimeout",
                        params: &[Param { name: "aFailoverTimeout", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute nsIProxyInfo failoverProxy; */
                    Method {
                        name: "GetFailoverProxy",
                        params: &[Param { name: "aFailoverProxy", ty: "*mut *const nsIProxyInfo" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetFailoverProxy",
                        params: &[Param { name: "aFailoverProxy", ty: "*const nsIProxyInfo" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute ACString proxyAuthorizationHeader; */
                    Method {
                        name: "GetProxyAuthorizationHeader",
                        params: &[Param { name: "aProxyAuthorizationHeader", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute ACString connectionIsolationKey; */
                    Method {
                        name: "GetConnectionIsolationKey",
                        params: &[Param { name: "aConnectionIsolationKey", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

