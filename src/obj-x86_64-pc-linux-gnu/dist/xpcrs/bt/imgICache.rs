//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/image/imgICache.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "imgICache",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void clearCache (in boolean chrome); */
                    Method {
                        name: "ClearCache",
                        params: &[Param { name: "chrome", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [noscript] void removeEntry (in nsIURI uri, [optional] in Document doc); */
                    Method {
                        name: "RemoveEntry",
                        params: &[Param { name: "uri", ty: "*const nsIURI" }, Param { name: "doc", ty: "*const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void removeEntriesFromPrincipal (in nsIPrincipal aPrincipal); */
                    Method {
                        name: "RemoveEntriesFromPrincipal",
                        params: &[Param { name: "aPrincipal", ty: "*const nsIPrincipal" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] nsIProperties findEntryProperties (in nsIURI uri, [optional] in Document doc); */
                    Method {
                        name: "FindEntryProperties",
                        params: &[Param { name: "uri", ty: "*const nsIURI" }, Param { name: "doc", ty: "*const libc::c_void" }, Param { name: "_retval", ty: "*mut*const nsIProperties" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void respectPrivacyNotifications (); */
                    Method {
                        name: "RespectPrivacyNotifications",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* [noscript,notxpcom] void clearCacheForControlledDocument (in Document doc); */
                    Method {
                        name: "ClearCacheForControlledDocument",
                        params: &[Param { name: "doc", ty: "*const libc::c_void" }],
                        ret: "libc::c_void",
                    },

                    ]),
        },

        ]; D}

