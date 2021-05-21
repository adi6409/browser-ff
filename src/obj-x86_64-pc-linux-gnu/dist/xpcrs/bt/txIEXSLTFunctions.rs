//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/xslt/xslt/txIEXSLTFunctions.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "txIEXSLTFunctions",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* DocumentFragment match (in AString str, in AString regex, in AString flags, in Document doc); */
                    Method {
                        name: "Match",
                        params: &[Param { name: "str", ty: "*const ::nsstring::nsAString" }, Param { name: "regex", ty: "*const ::nsstring::nsAString" }, Param { name: "flags", ty: "*const ::nsstring::nsAString" }, Param { name: "doc", ty: "*const libc::c_void" }, Param { name: "_retval", ty: "*mut *const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString replace (in AString str, in AString regex, in AString flags, in AString replace); */
                    Method {
                        name: "Replace",
                        params: &[Param { name: "str", ty: "*const ::nsstring::nsAString" }, Param { name: "regex", ty: "*const ::nsstring::nsAString" }, Param { name: "flags", ty: "*const ::nsstring::nsAString" }, Param { name: "replace", ty: "*const ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean test (in AString str, in AString regex, in AString flags); */
                    Method {
                        name: "Test",
                        params: &[Param { name: "str", ty: "*const ::nsstring::nsAString" }, Param { name: "regex", ty: "*const ::nsstring::nsAString" }, Param { name: "flags", ty: "*const ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

