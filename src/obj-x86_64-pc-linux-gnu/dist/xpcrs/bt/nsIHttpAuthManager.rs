//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/protocol/http/nsIHttpAuthManager.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIHttpAuthManager",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* [must_use] void getAuthIdentity (in ACString aScheme, in ACString aHost, in int32_t aPort, in ACString aAuthType, in ACString aRealm, in ACString aPath, out AString aUserDomain, out AString aUserName, out AString aUserPassword, [optional] in bool aIsPrivate, [optional] in nsIPrincipal aPrincipal); */
                    Method {
                        name: "GetAuthIdentity",
                        params: &[Param { name: "aScheme", ty: "*const ::nsstring::nsACString" }, Param { name: "aHost", ty: "*const ::nsstring::nsACString" }, Param { name: "aPort", ty: "int32_t" }, Param { name: "aAuthType", ty: "*const ::nsstring::nsACString" }, Param { name: "aRealm", ty: "*const ::nsstring::nsACString" }, Param { name: "aPath", ty: "*const ::nsstring::nsACString" }, Param { name: "aUserDomain", ty: "*mut ::nsstring::nsAString" }, Param { name: "aUserName", ty: "*mut ::nsstring::nsAString" }, Param { name: "aUserPassword", ty: "*mut ::nsstring::nsAString" }, Param { name: "aIsPrivate", ty: "bool" }, Param { name: "aPrincipal", ty: "*const nsIPrincipal" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void setAuthIdentity (in ACString aScheme, in ACString aHost, in int32_t aPort, in ACString aAuthType, in ACString aRealm, in ACString aPath, in AString aUserDomain, in AString aUserName, in AString aUserPassword, [optional] in boolean aIsPrivate, [optional] in nsIPrincipal aPrincipal); */
                    Method {
                        name: "SetAuthIdentity",
                        params: &[Param { name: "aScheme", ty: "*const ::nsstring::nsACString" }, Param { name: "aHost", ty: "*const ::nsstring::nsACString" }, Param { name: "aPort", ty: "int32_t" }, Param { name: "aAuthType", ty: "*const ::nsstring::nsACString" }, Param { name: "aRealm", ty: "*const ::nsstring::nsACString" }, Param { name: "aPath", ty: "*const ::nsstring::nsACString" }, Param { name: "aUserDomain", ty: "*const ::nsstring::nsAString" }, Param { name: "aUserName", ty: "*const ::nsstring::nsAString" }, Param { name: "aUserPassword", ty: "*const ::nsstring::nsAString" }, Param { name: "aIsPrivate", ty: "bool" }, Param { name: "aPrincipal", ty: "*const nsIPrincipal" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void clearAll (); */
                    Method {
                        name: "ClearAll",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

