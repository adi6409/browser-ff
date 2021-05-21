//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsIPipe.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPipe",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* [must_use] void init (in boolean nonBlockingInput, in boolean nonBlockingOutput, in unsigned long segmentSize, in unsigned long segmentCount); */
                    Method {
                        name: "Init",
                        params: &[Param { name: "nonBlockingInput", ty: "bool" }, Param { name: "nonBlockingOutput", ty: "bool" }, Param { name: "segmentSize", ty: "u32" }, Param { name: "segmentCount", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] readonly attribute nsIAsyncInputStream inputStream; */
                    Method {
                        name: "GetInputStream",
                        params: &[Param { name: "aInputStream", ty: "*mut*const nsIAsyncInputStream" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] readonly attribute nsIAsyncOutputStream outputStream; */
                    Method {
                        name: "GetOutputStream",
                        params: &[Param { name: "aOutputStream", ty: "*mut*const nsIAsyncOutputStream" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsISearchableInputStream",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void search (in string forString, in boolean ignoreCase, out boolean found, out unsigned long offsetSearchedTo); */
                    Method {
                        name: "Search",
                        params: &[Param { name: "forString", ty: "*const libc::c_char" }, Param { name: "ignoreCase", ty: "bool" }, Param { name: "found", ty: "*mut bool" }, Param { name: "offsetSearchedTo", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

