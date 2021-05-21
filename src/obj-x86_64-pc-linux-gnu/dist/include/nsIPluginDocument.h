/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/plugins/base/nsIPluginDocument.idl
 */

#ifndef __gen_nsIPluginDocument_h__
#define __gen_nsIPluginDocument_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsIStreamListener_h__
#include "nsIStreamListener.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIPluginDocument */
#define NS_IPLUGINDOCUMENT_IID_STR "a93a0f0f-24f0-4206-a21b-56a43dcbdd88"

#define NS_IPLUGINDOCUMENT_IID \
  {0xa93a0f0f, 0x24f0, 0x4206, \
    { 0xa2, 0x1b, 0x56, 0xa4, 0x3d, 0xcb, 0xdd, 0x88 }}

class NS_NO_VTABLE nsIPluginDocument : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPLUGINDOCUMENT_IID)

  /* void print (); */
  NS_IMETHOD Print(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPluginDocument, NS_IPLUGINDOCUMENT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPLUGINDOCUMENT \
  NS_IMETHOD Print(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPLUGINDOCUMENT \
  nsresult Print(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPLUGINDOCUMENT(_to) \
  NS_IMETHOD Print(void) override { return _to Print(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPLUGINDOCUMENT(_to) \
  NS_IMETHOD Print(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Print(); } 


#endif /* __gen_nsIPluginDocument_h__ */
