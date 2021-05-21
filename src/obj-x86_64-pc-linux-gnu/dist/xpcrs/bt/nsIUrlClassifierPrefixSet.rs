//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/url-classifier/nsIUrlClassifierPrefixSet.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIUrlClassifierPrefixSet",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void init (in ACString aName); */
                    Method {
                        name: "Init",
                        params: &[Param { name: "aName", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setPrefixes ([array, size_is (aLength), const] in unsigned long aPrefixes, in unsigned long aLength); */
                    Method {
                        name: "SetPrefixes",
                        params: &[Param { name: "aPrefixes", ty: "*const u32" }, Param { name: "aLength", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void getPrefixes (out unsigned long aCount, [array, size_is (aCount), retval] out unsigned long aPrefixes); */
                    Method {
                        name: "GetPrefixes",
                        params: &[Param { name: "aCount", ty: "*mut u32" }, Param { name: "aPrefixes", ty: "*mut *mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean contains (in unsigned long aPrefix); */
                    Method {
                        name: "Contains",
                        params: &[Param { name: "aPrefix", ty: "u32" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean isEmpty (); */
                    Method {
                        name: "IsEmpty",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

