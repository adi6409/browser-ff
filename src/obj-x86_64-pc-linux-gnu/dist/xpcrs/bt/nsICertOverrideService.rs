//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsICertOverrideService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsICertOverride",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute ACString asciiHost; */
                    Method {
                        name: "GetAsciiHost",
                        params: &[Param { name: "aAsciiHost", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute int32_t port; */
                    Method {
                        name: "GetPort",
                        params: &[Param { name: "aPort", ty: "*mut int32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean isTemporary; */
                    Method {
                        name: "GetIsTemporary",
                        params: &[Param { name: "aIsTemporary", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute ACString dbKey; */
                    Method {
                        name: "GetDbKey",
                        params: &[Param { name: "aDbKey", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute ACString hostPort; */
                    Method {
                        name: "GetHostPort",
                        params: &[Param { name: "aHostPort", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsICertOverrideService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* [must_use] void rememberValidityOverride (in AUTF8String aHostName, in int32_t aPort, in nsIX509Cert aCert, in uint32_t aOverrideBits, in boolean aTemporary); */
                    Method {
                        name: "RememberValidityOverride",
                        params: &[Param { name: "aHostName", ty: "*const ::nsstring::nsACString" }, Param { name: "aPort", ty: "int32_t" }, Param { name: "aCert", ty: "*const nsIX509Cert" }, Param { name: "aOverrideBits", ty: "uint32_t" }, Param { name: "aTemporary", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void rememberTemporaryValidityOverrideUsingFingerprint (in AUTF8String aHostName, in int32_t aPort, in AUTF8String aCertFingerprint, in uint32_t aOverrideBits); */
                    Method {
                        name: "RememberTemporaryValidityOverrideUsingFingerprint",
                        params: &[Param { name: "aHostName", ty: "*const ::nsstring::nsACString" }, Param { name: "aPort", ty: "int32_t" }, Param { name: "aCertFingerprint", ty: "*const ::nsstring::nsACString" }, Param { name: "aOverrideBits", ty: "uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] boolean hasMatchingOverride (in AUTF8String aHostName, in int32_t aPort, in nsIX509Cert aCert, out uint32_t aOverrideBits, out boolean aIsTemporary); */
                    Method {
                        name: "HasMatchingOverride",
                        params: &[Param { name: "aHostName", ty: "*const ::nsstring::nsACString" }, Param { name: "aPort", ty: "int32_t" }, Param { name: "aCert", ty: "*const nsIX509Cert" }, Param { name: "aOverrideBits", ty: "*mut uint32_t" }, Param { name: "aIsTemporary", ty: "*mut bool" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void clearValidityOverride (in AUTF8String aHostName, in int32_t aPort); */
                    Method {
                        name: "ClearValidityOverride",
                        params: &[Param { name: "aHostName", ty: "*const ::nsstring::nsACString" }, Param { name: "aPort", ty: "int32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void clearAllOverrides (); */
                    Method {
                        name: "ClearAllOverrides",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] uint32_t isCertUsedForOverrides (in nsIX509Cert aCert, in boolean aCheckTemporaries, in boolean aCheckPermanents); */
                    Method {
                        name: "IsCertUsedForOverrides",
                        params: &[Param { name: "aCert", ty: "*const nsIX509Cert" }, Param { name: "aCheckTemporaries", ty: "bool" }, Param { name: "aCheckPermanents", ty: "bool" }, Param { name: "_retval", ty: "*mut uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* Array<nsICertOverride> getOverrides (); */
                    Method {
                        name: "GetOverrides",
                        params: &[Param { name: "_retval", ty: "*mut thin_vec::ThinVec<RefPtr<nsICertOverride>>" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setDisableAllSecurityChecksAndLetAttackersInterceptMyData (in boolean aDisable); */
                    Method {
                        name: "SetDisableAllSecurityChecksAndLetAttackersInterceptMyData",
                        params: &[Param { name: "aDisable", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

