//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsIDirectoryEnumerator.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDirectoryEnumerator",
            base: Some("nsISimpleEnumerator"),
            methods: Ok(&[
                    /* readonly attribute nsIFile nextFile; */
                    Method {
                        name: "GetNextFile",
                        params: &[Param { name: "aNextFile", ty: "*mut*const nsIFile" }],
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

