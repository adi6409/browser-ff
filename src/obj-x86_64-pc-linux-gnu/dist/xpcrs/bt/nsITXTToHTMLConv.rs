//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/streamconv/nsITXTToHTMLConv.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsITXTToHTMLConv",
            base: Some("nsIStreamConverter"),
            methods: Ok(&[
                    /* void setTitle (in wstring text); */
                    Method {
                        name: "SetTitle",
                        params: &[Param { name: "text", ty: "*const i16" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void preFormatHTML (in boolean value); */
                    Method {
                        name: "PreFormatHTML",
                        params: &[Param { name: "value", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

