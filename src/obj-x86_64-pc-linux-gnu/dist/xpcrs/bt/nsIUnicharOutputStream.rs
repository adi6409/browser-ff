//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsIUnicharOutputStream.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIUnicharOutputStream",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* boolean write (in unsigned long aCount, [array, size_is (aCount), const] in char16_t c); */
                    Method {
                        name: "Write",
                        params: &[Param { name: "aCount", ty: "u32" }, Param { name: "c", ty: "*const char16_t" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean writeString (in AString str); */
                    Method {
                        name: "WriteString",
                        params: &[Param { name: "str", ty: "*const ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void flush (); */
                    Method {
                        name: "Flush",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void close (); */
                    Method {
                        name: "Close",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

