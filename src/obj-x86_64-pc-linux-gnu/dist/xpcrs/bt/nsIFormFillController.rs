//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/satchel/nsIFormFillController.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIFormFillController",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute HTMLInputElement focusedInput; */
                    Method {
                        name: "GetFocusedInput",
                        params: &[Param { name: "aFocusedInput", ty: "*mut *const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean passwordPopupAutomaticallyOpened; */
                    Method {
                        name: "GetPasswordPopupAutomaticallyOpened",
                        params: &[Param { name: "aPasswordPopupAutomaticallyOpened", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void attachPopupElementToDocument (in Document document, in Element popup); */
                    Method {
                        name: "AttachPopupElementToDocument",
                        params: &[Param { name: "document", ty: "*const libc::c_void" }, Param { name: "popup", ty: "*const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void detachFromDocument (in Document document); */
                    Method {
                        name: "DetachFromDocument",
                        params: &[Param { name: "document", ty: "*const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] void markAsLoginManagerField (in HTMLInputElement aInput); */
                    Method {
                        name: "MarkAsLoginManagerField",
                        params: &[Param { name: "aInput", ty: "*const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] void markAsAutofillField (in HTMLInputElement aInput); */
                    Method {
                        name: "MarkAsAutofillField",
                        params: &[Param { name: "aInput", ty: "*const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] void showPopup (); */
                    Method {
                        name: "ShowPopup",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

