//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsIColorPicker.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIColorPickerShownCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void update (in AString color); */
                    Method {
                        name: "Update",
                        params: &[Param { name: "color", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void done (in AString color); */
                    Method {
                        name: "Done",
                        params: &[Param { name: "color", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIColorPicker",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void init (in mozIDOMWindowProxy parent, in AString title, in AString initialColor); */
                    Method {
                        name: "Init",
                        params: &[Param { name: "parent", ty: "*const mozIDOMWindowProxy" }, Param { name: "title", ty: "*const ::nsstring::nsAString" }, Param { name: "initialColor", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void open (in nsIColorPickerShownCallback aColorPickerShownCallback); */
                    Method {
                        name: "Open",
                        params: &[Param { name: "aColorPickerShownCallback", ty: "*const nsIColorPickerShownCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

