/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsIUnicharLineInputStream.idl
 */

#ifndef __gen_nsIUnicharLineInputStream_h__
#define __gen_nsIUnicharLineInputStream_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIUnicharLineInputStream */
#define NS_IUNICHARLINEINPUTSTREAM_IID_STR "67f42475-ba80-40f8-ac0b-649c89230184"

#define NS_IUNICHARLINEINPUTSTREAM_IID \
  {0x67f42475, 0xba80, 0x40f8, \
    { 0xac, 0x0b, 0x64, 0x9c, 0x89, 0x23, 0x01, 0x84 }}

class NS_NO_VTABLE nsIUnicharLineInputStream : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IUNICHARLINEINPUTSTREAM_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIUnicharLineInputStream;

  /* boolean readLine (out AString aLine); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ReadLine(nsAString& aLine, bool *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIUnicharLineInputStream, NS_IUNICHARLINEINPUTSTREAM_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIUNICHARLINEINPUTSTREAM \
  NS_IMETHOD ReadLine(nsAString& aLine, bool *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIUNICHARLINEINPUTSTREAM \
  nsresult ReadLine(nsAString& aLine, bool *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIUNICHARLINEINPUTSTREAM(_to) \
  NS_IMETHOD ReadLine(nsAString& aLine, bool *_retval) override { return _to ReadLine(aLine, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIUNICHARLINEINPUTSTREAM(_to) \
  NS_IMETHOD ReadLine(nsAString& aLine, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ReadLine(aLine, _retval); } 


#endif /* __gen_nsIUnicharLineInputStream_h__ */
