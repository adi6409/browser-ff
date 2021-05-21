/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/protocol/file/nsIFileProtocolHandler.idl
 */

#ifndef __gen_nsIFileProtocolHandler_h__
#define __gen_nsIFileProtocolHandler_h__


#ifndef __gen_nsIProtocolHandler_h__
#include "nsIProtocolHandler.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIFile; /* forward declaration */

class nsIURIMutator; /* forward declaration */


/* starting interface:    nsIFileProtocolHandler */
#define NS_IFILEPROTOCOLHANDLER_IID_STR "1fb25bd5-4354-4dcd-8d97-621b7b3ed2e4"

#define NS_IFILEPROTOCOLHANDLER_IID \
  {0x1fb25bd5, 0x4354, 0x4dcd, \
    { 0x8d, 0x97, 0x62, 0x1b, 0x7b, 0x3e, 0xd2, 0xe4 }}

class NS_NO_VTABLE nsIFileProtocolHandler : public nsIProtocolHandler {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IFILEPROTOCOLHANDLER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIFileProtocolHandler;

  /* nsIURI newFileURI (in nsIFile aFile); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD NewFileURI(nsIFile *aFile, nsIURI **_retval) = 0;

  /* nsIURIMutator newFileURIMutator (in nsIFile file); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD NewFileURIMutator(nsIFile *file, nsIURIMutator **_retval) = 0;

  /* AUTF8String getURLSpecFromFile (in nsIFile file); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetURLSpecFromFile(nsIFile *file, nsACString& _retval) = 0;

  /* AUTF8String getURLSpecFromActualFile (in nsIFile file); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetURLSpecFromActualFile(nsIFile *file, nsACString& _retval) = 0;

  /* AUTF8String getURLSpecFromDir (in nsIFile file); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetURLSpecFromDir(nsIFile *file, nsACString& _retval) = 0;

  /* nsIFile getFileFromURLSpec (in AUTF8String url); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFileFromURLSpec(const nsACString& url, nsIFile **_retval) = 0;

  /* nsIURI readURLFile (in nsIFile file); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ReadURLFile(nsIFile *file, nsIURI **_retval) = 0;

  /* nsIURI readShellLink (in nsIFile file); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ReadShellLink(nsIFile *file, nsIURI **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIFileProtocolHandler, NS_IFILEPROTOCOLHANDLER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIFILEPROTOCOLHANDLER \
  NS_IMETHOD NewFileURI(nsIFile *aFile, nsIURI **_retval) override; \
  NS_IMETHOD NewFileURIMutator(nsIFile *file, nsIURIMutator **_retval) override; \
  NS_IMETHOD GetURLSpecFromFile(nsIFile *file, nsACString& _retval) override; \
  NS_IMETHOD GetURLSpecFromActualFile(nsIFile *file, nsACString& _retval) override; \
  NS_IMETHOD GetURLSpecFromDir(nsIFile *file, nsACString& _retval) override; \
  NS_IMETHOD GetFileFromURLSpec(const nsACString& url, nsIFile **_retval) override; \
  NS_IMETHOD ReadURLFile(nsIFile *file, nsIURI **_retval) override; \
  NS_IMETHOD ReadShellLink(nsIFile *file, nsIURI **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIFILEPROTOCOLHANDLER \
  nsresult NewFileURI(nsIFile *aFile, nsIURI **_retval); \
  nsresult NewFileURIMutator(nsIFile *file, nsIURIMutator **_retval); \
  nsresult GetURLSpecFromFile(nsIFile *file, nsACString& _retval); \
  nsresult GetURLSpecFromActualFile(nsIFile *file, nsACString& _retval); \
  nsresult GetURLSpecFromDir(nsIFile *file, nsACString& _retval); \
  nsresult GetFileFromURLSpec(const nsACString& url, nsIFile **_retval); \
  nsresult ReadURLFile(nsIFile *file, nsIURI **_retval); \
  nsresult ReadShellLink(nsIFile *file, nsIURI **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIFILEPROTOCOLHANDLER(_to) \
  NS_IMETHOD NewFileURI(nsIFile *aFile, nsIURI **_retval) override { return _to NewFileURI(aFile, _retval); } \
  NS_IMETHOD NewFileURIMutator(nsIFile *file, nsIURIMutator **_retval) override { return _to NewFileURIMutator(file, _retval); } \
  NS_IMETHOD GetURLSpecFromFile(nsIFile *file, nsACString& _retval) override { return _to GetURLSpecFromFile(file, _retval); } \
  NS_IMETHOD GetURLSpecFromActualFile(nsIFile *file, nsACString& _retval) override { return _to GetURLSpecFromActualFile(file, _retval); } \
  NS_IMETHOD GetURLSpecFromDir(nsIFile *file, nsACString& _retval) override { return _to GetURLSpecFromDir(file, _retval); } \
  NS_IMETHOD GetFileFromURLSpec(const nsACString& url, nsIFile **_retval) override { return _to GetFileFromURLSpec(url, _retval); } \
  NS_IMETHOD ReadURLFile(nsIFile *file, nsIURI **_retval) override { return _to ReadURLFile(file, _retval); } \
  NS_IMETHOD ReadShellLink(nsIFile *file, nsIURI **_retval) override { return _to ReadShellLink(file, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIFILEPROTOCOLHANDLER(_to) \
  NS_IMETHOD NewFileURI(nsIFile *aFile, nsIURI **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NewFileURI(aFile, _retval); } \
  NS_IMETHOD NewFileURIMutator(nsIFile *file, nsIURIMutator **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NewFileURIMutator(file, _retval); } \
  NS_IMETHOD GetURLSpecFromFile(nsIFile *file, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetURLSpecFromFile(file, _retval); } \
  NS_IMETHOD GetURLSpecFromActualFile(nsIFile *file, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetURLSpecFromActualFile(file, _retval); } \
  NS_IMETHOD GetURLSpecFromDir(nsIFile *file, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetURLSpecFromDir(file, _retval); } \
  NS_IMETHOD GetFileFromURLSpec(const nsACString& url, nsIFile **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFileFromURLSpec(url, _retval); } \
  NS_IMETHOD ReadURLFile(nsIFile *file, nsIURI **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ReadURLFile(file, _retval); } \
  NS_IMETHOD ReadShellLink(nsIFile *file, nsIURI **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ReadShellLink(file, _retval); } 


#endif /* __gen_nsIFileProtocolHandler_h__ */
