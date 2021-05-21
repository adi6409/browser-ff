//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsISystemStatusBar.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISystemStatusBar",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void addItem (in Element aMenuElement); */
                    Method {
                        name: "AddItem",
                        params: &[Param { name: "aMenuElement", ty: "*const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void removeItem (in Element aMenuElement); */
                    Method {
                        name: "RemoveItem",
                        params: &[Param { name: "aMenuElement", ty: "*const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

