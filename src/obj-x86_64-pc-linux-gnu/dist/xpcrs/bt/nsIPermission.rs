//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIPermission.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPermission",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute nsIPrincipal principal; */
                    Method {
                        name: "GetPrincipal",
                        params: &[Param { name: "aPrincipal", ty: "*mut*const nsIPrincipal" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute ACString type; */
                    Method {
                        name: "GetType",
                        params: &[Param { name: "aType", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute uint32_t capability; */
                    Method {
                        name: "GetCapability",
                        params: &[Param { name: "aCapability", ty: "*mut uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute uint32_t expireType; */
                    Method {
                        name: "GetExpireType",
                        params: &[Param { name: "aExpireType", ty: "*mut uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute int64_t expireTime; */
                    Method {
                        name: "GetExpireTime",
                        params: &[Param { name: "aExpireTime", ty: "*mut int64_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute int64_t modificationTime; */
                    Method {
                        name: "GetModificationTime",
                        params: &[Param { name: "aModificationTime", ty: "*mut int64_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean matches (in nsIPrincipal principal, in boolean exactHost); */
                    Method {
                        name: "Matches",
                        params: &[Param { name: "principal", ty: "*const nsIPrincipal" }, Param { name: "exactHost", ty: "bool" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [noscript] boolean matchesPrincipalForPermission (in nsIPrincipal principal, in boolean exactHost); */
                    Method {
                        name: "MatchesPrincipalForPermission",
                        params: &[Param { name: "principal", ty: "*const nsIPrincipal" }, Param { name: "exactHost", ty: "bool" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean matchesURI (in nsIURI uri, in boolean exactHost); */
                    Method {
                        name: "MatchesURI",
                        params: &[Param { name: "uri", ty: "*const nsIURI" }, Param { name: "exactHost", ty: "bool" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

