//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/ds/nsIMutableArray.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIMutableArray",
            base: Some("nsIArrayExtensions"),
            methods: Ok(&[
                    /* void appendElement (in nsISupports element); */
                    Method {
                        name: "AppendElement",
                        params: &[Param { name: "element", ty: "*const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void removeElementAt (in unsigned long index); */
                    Method {
                        name: "RemoveElementAt",
                        params: &[Param { name: "index", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void insertElementAt (in nsISupports element, in unsigned long index); */
                    Method {
                        name: "InsertElementAt",
                        params: &[Param { name: "element", ty: "*const nsISupports" }, Param { name: "index", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void replaceElementAt (in nsISupports element, in unsigned long index); */
                    Method {
                        name: "ReplaceElementAt",
                        params: &[Param { name: "element", ty: "*const nsISupports" }, Param { name: "index", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void clear (); */
                    Method {
                        name: "Clear",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

