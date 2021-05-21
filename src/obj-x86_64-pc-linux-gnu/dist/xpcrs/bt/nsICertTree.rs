//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsICertTree.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsICertTreeItem",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* [must_use] readonly attribute nsIX509Cert cert; */
                    Method {
                        name: "GetCert",
                        params: &[Param { name: "aCert", ty: "*mut*const nsIX509Cert" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsICertTree",
            base: Some("nsITreeView"),
            methods: Ok(&[
                    /* [must_use] void loadCertsFromCache (in Array<nsIX509Cert> cache, in unsigned long type); */
                    Method {
                        name: "LoadCertsFromCache",
                        params: &[Param { name: "cache", ty: "*const thin_vec::ThinVec<RefPtr<nsIX509Cert>>" }, Param { name: "type_", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] nsIX509Cert getCert (in unsigned long index); */
                    Method {
                        name: "GetCert",
                        params: &[Param { name: "index", ty: "u32" }, Param { name: "_retval", ty: "*mut*const nsIX509Cert" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] nsICertTreeItem getTreeItem (in unsigned long index); */
                    Method {
                        name: "GetTreeItem",
                        params: &[Param { name: "index", ty: "u32" }, Param { name: "_retval", ty: "*mut *const nsICertTreeItem" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void deleteEntryObject (in unsigned long index); */
                    Method {
                        name: "DeleteEntryObject",
                        params: &[Param { name: "index", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

