//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsIIOUtil.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIIOUtil",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* boolean inputStreamIsBuffered (in nsIInputStream aStream); */
                    Method {
                        name: "InputStreamIsBuffered",
                        params: &[Param { name: "aStream", ty: "*const nsIInputStream" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean outputStreamIsBuffered (in nsIOutputStream aStream); */
                    Method {
                        name: "OutputStreamIsBuffered",
                        params: &[Param { name: "aStream", ty: "*const nsIOutputStream" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

