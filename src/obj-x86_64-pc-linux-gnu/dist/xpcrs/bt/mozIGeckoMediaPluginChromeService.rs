//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/media/gmp/mozIGeckoMediaPluginChromeService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "mozIGeckoMediaPluginChromeService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void addPluginDirectory (in AString directory); */
                    Method {
                        name: "AddPluginDirectory",
                        params: &[Param { name: "directory", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void removePluginDirectory (in AString directory); */
                    Method {
                        name: "RemovePluginDirectory",
                        params: &[Param { name: "directory", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void removeAndDeletePluginDirectory (in AString directory, [optional] in bool defer); */
                    Method {
                        name: "RemoveAndDeletePluginDirectory",
                        params: &[Param { name: "directory", ty: "*const ::nsstring::nsAString" }, Param { name: "defer", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void forgetThisSite (in AString site, in AString aPattern); */
                    Method {
                        name: "ForgetThisSite",
                        params: &[Param { name: "site", ty: "*const ::nsstring::nsAString" }, Param { name: "aPattern", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* bool isPersistentStorageAllowed (in ACString nodeId); */
                    Method {
                        name: "IsPersistentStorageAllowed",
                        params: &[Param { name: "nodeId", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIFile getStorageDir (); */
                    Method {
                        name: "GetStorageDir",
                        params: &[Param { name: "_retval", ty: "*mut *const nsIFile" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

