//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/parser/html/nsIParserUtils.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIParserUtils",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* AString sanitize (in AString src, in unsigned long flags); */
                    Method {
                        name: "Sanitize",
                        params: &[Param { name: "src", ty: "*const ::nsstring::nsAString" }, Param { name: "flags", ty: "u32" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString convertToPlainText (in AString src, in unsigned long flags, in unsigned long wrapCol); */
                    Method {
                        name: "ConvertToPlainText",
                        params: &[Param { name: "src", ty: "*const ::nsstring::nsAString" }, Param { name: "flags", ty: "u32" }, Param { name: "wrapCol", ty: "u32" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* DocumentFragment parseFragment (in AString fragment, in unsigned long flags, in boolean isXML, in nsIURI baseURI, in Element element); */
                    Method {
                        name: "ParseFragment",
                        params: &[Param { name: "fragment", ty: "*const ::nsstring::nsAString" }, Param { name: "flags", ty: "u32" }, Param { name: "isXML", ty: "bool" }, Param { name: "baseURI", ty: "*const nsIURI" }, Param { name: "element", ty: "*const libc::c_void" }, Param { name: "_retval", ty: "*mut *const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

