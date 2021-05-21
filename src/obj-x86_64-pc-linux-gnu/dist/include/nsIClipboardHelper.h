/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsIClipboardHelper.idl
 */

#ifndef __gen_nsIClipboardHelper_h__
#define __gen_nsIClipboardHelper_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsIClipboard_h__
#include "nsIClipboard.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
#include "nsString.h" // needed for AString -> nsAString, unfortunately

/* starting interface:    nsIClipboardHelper */
#define NS_ICLIPBOARDHELPER_IID_STR "438307fd-0c68-4d79-922a-f6cc9550cd02"

#define NS_ICLIPBOARDHELPER_IID \
  {0x438307fd, 0x0c68, 0x4d79, \
    { 0x92, 0x2a, 0xf6, 0xcc, 0x95, 0x50, 0xcd, 0x02 }}

class NS_NO_VTABLE nsIClipboardHelper : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICLIPBOARDHELPER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIClipboardHelper;

  /* void copyStringToClipboard (in AString aString, in long aClipboardID); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CopyStringToClipboard(const nsAString& aString, int32_t aClipboardID) = 0;

  /* void copyString (in AString aString); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CopyString(const nsAString& aString) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIClipboardHelper, NS_ICLIPBOARDHELPER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICLIPBOARDHELPER \
  NS_IMETHOD CopyStringToClipboard(const nsAString& aString, int32_t aClipboardID) override; \
  NS_IMETHOD CopyString(const nsAString& aString) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICLIPBOARDHELPER \
  nsresult CopyStringToClipboard(const nsAString& aString, int32_t aClipboardID); \
  nsresult CopyString(const nsAString& aString); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICLIPBOARDHELPER(_to) \
  NS_IMETHOD CopyStringToClipboard(const nsAString& aString, int32_t aClipboardID) override { return _to CopyStringToClipboard(aString, aClipboardID); } \
  NS_IMETHOD CopyString(const nsAString& aString) override { return _to CopyString(aString); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICLIPBOARDHELPER(_to) \
  NS_IMETHOD CopyStringToClipboard(const nsAString& aString, int32_t aClipboardID) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CopyStringToClipboard(aString, aClipboardID); } \
  NS_IMETHOD CopyString(const nsAString& aString) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CopyString(aString); } 


#endif /* __gen_nsIClipboardHelper_h__ */
