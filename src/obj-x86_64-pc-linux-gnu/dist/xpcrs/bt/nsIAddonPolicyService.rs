//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/caps/nsIAddonPolicyService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAddonPolicyService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute AString defaultCSP; */
                    Method {
                        name: "GetDefaultCSP",
                        params: &[Param { name: "aDefaultCSP", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString getBaseCSP (in AString aAddonId); */
                    Method {
                        name: "GetBaseCSP",
                        params: &[Param { name: "aAddonId", ty: "*const ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString getExtensionPageCSP (in AString aAddonId); */
                    Method {
                        name: "GetExtensionPageCSP",
                        params: &[Param { name: "aAddonId", ty: "*const ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* ACString getGeneratedBackgroundPageUrl (in ACString aAddonId); */
                    Method {
                        name: "GetGeneratedBackgroundPageUrl",
                        params: &[Param { name: "aAddonId", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean addonHasPermission (in AString aAddonId, in AString aPerm); */
                    Method {
                        name: "AddonHasPermission",
                        params: &[Param { name: "aAddonId", ty: "*const ::nsstring::nsAString" }, Param { name: "aPerm", ty: "*const ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean addonMayLoadURI (in AString aAddonId, in nsIURI aURI, [optional] in boolean aExplicit); */
                    Method {
                        name: "AddonMayLoadURI",
                        params: &[Param { name: "aAddonId", ty: "*const ::nsstring::nsAString" }, Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aExplicit", ty: "bool" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString getExtensionName (in AString aAddonId); */
                    Method {
                        name: "GetExtensionName",
                        params: &[Param { name: "aAddonId", ty: "*const ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean extensionURILoadableByAnyone (in nsIURI aURI); */
                    Method {
                        name: "ExtensionURILoadableByAnyone",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString extensionURIToAddonId (in nsIURI aURI); */
                    Method {
                        name: "ExtensionURIToAddonId",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIAddonContentPolicy",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* AString validateAddonCSP (in AString aPolicyString, in unsigned long aPermittedPolicy); */
                    Method {
                        name: "ValidateAddonCSP",
                        params: &[Param { name: "aPolicyString", ty: "*const ::nsstring::nsAString" }, Param { name: "aPermittedPolicy", ty: "u32" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

