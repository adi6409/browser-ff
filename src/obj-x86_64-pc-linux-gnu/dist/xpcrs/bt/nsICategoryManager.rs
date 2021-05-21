//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/components/nsICategoryManager.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsICategoryEntry",
            base: Some("nsISupportsCString"),
            methods: Ok(&[
                    /* readonly attribute ACString entry; */
                    Method {
                        name: "GetEntry",
                        params: &[Param { name: "aEntry", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute ACString value; */
                    Method {
                        name: "GetValue",
                        params: &[Param { name: "aValue", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsICategoryManager",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* ACString getCategoryEntry (in ACString aCategory, in ACString aEntry); */
                    Method {
                        name: "GetCategoryEntry",
                        params: &[Param { name: "aCategory", ty: "*const ::nsstring::nsACString" }, Param { name: "aEntry", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* ACString addCategoryEntry (in ACString aCategory, in ACString aEntry, in ACString aValue, in boolean aPersist, in boolean aReplace); */
                    Method {
                        name: "AddCategoryEntry",
                        params: &[Param { name: "aCategory", ty: "*const ::nsstring::nsACString" }, Param { name: "aEntry", ty: "*const ::nsstring::nsACString" }, Param { name: "aValue", ty: "*const ::nsstring::nsACString" }, Param { name: "aPersist", ty: "bool" }, Param { name: "aReplace", ty: "bool" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void deleteCategoryEntry (in ACString aCategory, in ACString aEntry, in boolean aPersist); */
                    Method {
                        name: "DeleteCategoryEntry",
                        params: &[Param { name: "aCategory", ty: "*const ::nsstring::nsACString" }, Param { name: "aEntry", ty: "*const ::nsstring::nsACString" }, Param { name: "aPersist", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void deleteCategory (in ACString aCategory); */
                    Method {
                        name: "DeleteCategory",
                        params: &[Param { name: "aCategory", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsISimpleEnumerator enumerateCategory (in ACString aCategory); */
                    Method {
                        name: "EnumerateCategory",
                        params: &[Param { name: "aCategory", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut*const nsISimpleEnumerator" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsISimpleEnumerator enumerateCategories (); */
                    Method {
                        name: "EnumerateCategories",
                        params: &[Param { name: "_retval", ty: "*mut*const nsISimpleEnumerator" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

