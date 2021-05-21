//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/xul/nsIDOMXULContainerElement.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMXULContainerItemElement",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute Element parentContainer; */
                    Method {
                        name: "GetParentContainer",
                        params: &[Param { name: "aParentContainer", ty: "*mut *const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIDOMXULContainerElement",
            base: Some("nsIDOMXULContainerItemElement"),
            methods: Ok(&[
                    ]),
        },

        ]; D}

