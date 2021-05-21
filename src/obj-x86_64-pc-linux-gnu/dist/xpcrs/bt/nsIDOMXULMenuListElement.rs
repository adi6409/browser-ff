//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/xul/nsIDOMXULMenuListElement.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMXULMenuListElement",
            base: Some("nsIDOMXULSelectControlElement"),
            methods: Ok(&[
                    /* attribute boolean editable; */
                    Method {
                        name: "GetEditable",
                        params: &[Param { name: "aEditable", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetEditable",
                        params: &[Param { name: "aEditable", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute boolean open; */
                    Method {
                        name: "GetOpen",
                        params: &[Param { name: "aOpen", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetOpen",
                        params: &[Param { name: "aOpen", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString label; */
                    Method {
                        name: "GetLabel",
                        params: &[Param { name: "aLabel", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute AString crop; */
                    Method {
                        name: "GetCrop",
                        params: &[Param { name: "aCrop", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetCrop",
                        params: &[Param { name: "aCrop", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute AString image; */
                    Method {
                        name: "GetImage",
                        params: &[Param { name: "aImage", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetImage",
                        params: &[Param { name: "aImage", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute Element inputField; */
                    Method {
                        name: "GetInputField",
                        params: &[Param { name: "aInputField", ty: "*mut *const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

