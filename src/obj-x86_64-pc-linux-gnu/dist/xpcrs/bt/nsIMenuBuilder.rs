//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/html/nsIMenuBuilder.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIMenuBuilder",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void openContainer (in AString aLabel); */
                    Method {
                        name: "OpenContainer",
                        params: &[Param { name: "aLabel", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void addItemFor (in Element aElement, in boolean aCanLoadIcon); */
                    Method {
                        name: "AddItemFor",
                        params: &[Param { name: "aElement", ty: "*const libc::c_void" }, Param { name: "aCanLoadIcon", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void addSeparator (); */
                    Method {
                        name: "AddSeparator",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void undoAddSeparator (); */
                    Method {
                        name: "UndoAddSeparator",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void closeContainer (); */
                    Method {
                        name: "CloseContainer",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* AString toJSONString (); */
                    Method {
                        name: "ToJSONString",
                        params: &[Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void click (in AString aGeneratedItemId); */
                    Method {
                        name: "Click",
                        params: &[Param { name: "aGeneratedItemId", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

