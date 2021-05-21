//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/plugins/base/nsIHTTPHeaderListener.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIHTTPHeaderListener",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void newResponseHeader (in string headerName, in string headerValue); */
                    Method {
                        name: "NewResponseHeader",
                        params: &[Param { name: "headerName", ty: "*const libc::c_char" }, Param { name: "headerValue", ty: "*const libc::c_char" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void statusLine (in string line); */
                    Method {
                        name: "StatusLine",
                        params: &[Param { name: "line", ty: "*const libc::c_char" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

