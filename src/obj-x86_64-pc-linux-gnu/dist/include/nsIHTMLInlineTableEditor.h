/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/editor/nsIHTMLInlineTableEditor.idl
 */

#ifndef __gen_nsIHTMLInlineTableEditor_h__
#define __gen_nsIHTMLInlineTableEditor_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_domstubs_h__
#include "domstubs.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIHTMLInlineTableEditor */
#define NS_IHTMLINLINETABLEEDITOR_IID_STR "eda2e65c-a758-451f-9b05-77cb8de74ed2"

#define NS_IHTMLINLINETABLEEDITOR_IID \
  {0xeda2e65c, 0xa758, 0x451f, \
    { 0x9b, 0x05, 0x77, 0xcb, 0x8d, 0xe7, 0x4e, 0xd2 }}

class NS_NO_VTABLE nsIHTMLInlineTableEditor : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IHTMLINLINETABLEEDITOR_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIHTMLInlineTableEditor;

  /* [can_run_script] attribute boolean inlineTableEditingEnabled; */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD GetInlineTableEditingEnabled(bool *aInlineTableEditingEnabled) = 0;
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SetInlineTableEditingEnabled(bool aInlineTableEditingEnabled) = 0;

  /* [can_run_script] void refreshInlineTableEditingUI (); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD RefreshInlineTableEditingUI(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIHTMLInlineTableEditor, NS_IHTMLINLINETABLEEDITOR_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIHTMLINLINETABLEEDITOR \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD GetInlineTableEditingEnabled(bool *aInlineTableEditingEnabled) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SetInlineTableEditingEnabled(bool aInlineTableEditingEnabled) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD RefreshInlineTableEditingUI(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIHTMLINLINETABLEEDITOR \
  MOZ_CAN_RUN_SCRIPT nsresult GetInlineTableEditingEnabled(bool *aInlineTableEditingEnabled); \
  MOZ_CAN_RUN_SCRIPT nsresult SetInlineTableEditingEnabled(bool aInlineTableEditingEnabled); \
  MOZ_CAN_RUN_SCRIPT nsresult RefreshInlineTableEditingUI(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIHTMLINLINETABLEEDITOR(_to) \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD GetInlineTableEditingEnabled(bool *aInlineTableEditingEnabled) override { return _to GetInlineTableEditingEnabled(aInlineTableEditingEnabled); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SetInlineTableEditingEnabled(bool aInlineTableEditingEnabled) override { return _to SetInlineTableEditingEnabled(aInlineTableEditingEnabled); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD RefreshInlineTableEditingUI(void) override { return _to RefreshInlineTableEditingUI(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIHTMLINLINETABLEEDITOR(_to) \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD GetInlineTableEditingEnabled(bool *aInlineTableEditingEnabled) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetInlineTableEditingEnabled(aInlineTableEditingEnabled); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SetInlineTableEditingEnabled(bool aInlineTableEditingEnabled) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetInlineTableEditingEnabled(aInlineTableEditingEnabled); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD RefreshInlineTableEditingUI(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RefreshInlineTableEditingUI(); } 


#endif /* __gen_nsIHTMLInlineTableEditor_h__ */
