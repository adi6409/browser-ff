/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessibleEditableText.idl
 */

#ifndef __gen_nsIAccessibleEditableText_h__
#define __gen_nsIAccessibleEditableText_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIAccessibleEditableText */
#define NS_IACCESSIBLEEDITABLETEXT_IID_STR "28915cca-3366-4034-ba1d-b7afb9b37639"

#define NS_IACCESSIBLEEDITABLETEXT_IID \
  {0x28915cca, 0x3366, 0x4034, \
    { 0xba, 0x1d, 0xb7, 0xaf, 0xb9, 0xb3, 0x76, 0x39 }}

class NS_NO_VTABLE nsIAccessibleEditableText : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IACCESSIBLEEDITABLETEXT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIAccessibleEditableText;

  /* void setTextContents (in AString text); */
  NS_IMETHOD SetTextContents(const nsAString& text) = 0;

  /* void insertText (in AString text, in long position); */
  NS_IMETHOD InsertText(const nsAString& text, int32_t position) = 0;

  /* void copyText (in long startPos, in long endPos); */
  NS_IMETHOD CopyText(int32_t startPos, int32_t endPos) = 0;

  /* void cutText (in long startPos, in long endPos); */
  NS_IMETHOD CutText(int32_t startPos, int32_t endPos) = 0;

  /* void deleteText (in long startPos, in long endPos); */
  NS_IMETHOD DeleteText(int32_t startPos, int32_t endPos) = 0;

  /* [can_run_script] void pasteText (in long position); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD PasteText(int32_t position) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIAccessibleEditableText, NS_IACCESSIBLEEDITABLETEXT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIACCESSIBLEEDITABLETEXT \
  NS_IMETHOD SetTextContents(const nsAString& text) override; \
  NS_IMETHOD InsertText(const nsAString& text, int32_t position) override; \
  NS_IMETHOD CopyText(int32_t startPos, int32_t endPos) override; \
  NS_IMETHOD CutText(int32_t startPos, int32_t endPos) override; \
  NS_IMETHOD DeleteText(int32_t startPos, int32_t endPos) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD PasteText(int32_t position) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIACCESSIBLEEDITABLETEXT \
  nsresult SetTextContents(const nsAString& text); \
  nsresult InsertText(const nsAString& text, int32_t position); \
  nsresult CopyText(int32_t startPos, int32_t endPos); \
  nsresult CutText(int32_t startPos, int32_t endPos); \
  nsresult DeleteText(int32_t startPos, int32_t endPos); \
  MOZ_CAN_RUN_SCRIPT nsresult PasteText(int32_t position); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIACCESSIBLEEDITABLETEXT(_to) \
  NS_IMETHOD SetTextContents(const nsAString& text) override { return _to SetTextContents(text); } \
  NS_IMETHOD InsertText(const nsAString& text, int32_t position) override { return _to InsertText(text, position); } \
  NS_IMETHOD CopyText(int32_t startPos, int32_t endPos) override { return _to CopyText(startPos, endPos); } \
  NS_IMETHOD CutText(int32_t startPos, int32_t endPos) override { return _to CutText(startPos, endPos); } \
  NS_IMETHOD DeleteText(int32_t startPos, int32_t endPos) override { return _to DeleteText(startPos, endPos); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD PasteText(int32_t position) override { return _to PasteText(position); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIACCESSIBLEEDITABLETEXT(_to) \
  NS_IMETHOD SetTextContents(const nsAString& text) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetTextContents(text); } \
  NS_IMETHOD InsertText(const nsAString& text, int32_t position) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InsertText(text, position); } \
  NS_IMETHOD CopyText(int32_t startPos, int32_t endPos) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CopyText(startPos, endPos); } \
  NS_IMETHOD CutText(int32_t startPos, int32_t endPos) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CutText(startPos, endPos); } \
  NS_IMETHOD DeleteText(int32_t startPos, int32_t endPos) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DeleteText(startPos, endPos); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD PasteText(int32_t position) override { return !_to ? NS_ERROR_NULL_POINTER : _to->PasteText(position); } 


#endif /* __gen_nsIAccessibleEditableText_h__ */
