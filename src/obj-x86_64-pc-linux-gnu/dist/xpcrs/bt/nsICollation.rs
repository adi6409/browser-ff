//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/intl/locale/nsICollation.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsICollationFactory",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* nsICollation CreateCollation (); */
                    Method {
                        name: "CreateCollation",
                        params: &[Param { name: "_retval", ty: "*mut*const nsICollation" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsICollation CreateCollationForLocale (in ACString locale); */
                    Method {
                        name: "CreateCollationForLocale",
                        params: &[Param { name: "locale", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut*const nsICollation" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsICollation",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void initialize (in ACString locale); */
                    Method {
                        name: "Initialize",
                        params: &[Param { name: "locale", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* long compareString (in long strength, in AString string1, in AString string2); */
                    Method {
                        name: "CompareString",
                        params: &[Param { name: "strength", ty: "i32" }, Param { name: "string1", ty: "*const ::nsstring::nsAString" }, Param { name: "string2", ty: "*const ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [noscript] Array<octet> allocateRawSortKey (in long strength, in AString stringIn); */
                    Method {
                        name: "AllocateRawSortKey",
                        params: &[Param { name: "strength", ty: "i32" }, Param { name: "stringIn", ty: "*const ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut thin_vec::ThinVec<u8>" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [noscript] long compareRawSortKey (in Array<octet> key1, in Array<octet> key2); */
                    Method {
                        name: "CompareRawSortKey",
                        params: &[Param { name: "key1", ty: "*const thin_vec::ThinVec<u8>" }, Param { name: "key2", ty: "*const thin_vec::ThinVec<u8>" }, Param { name: "_retval", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

