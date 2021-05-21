/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIURLParser.idl
 */

#ifndef __gen_nsIURLParser_h__
#define __gen_nsIURLParser_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIURLParser */
#define NS_IURLPARSER_IID_STR "78c5d19f-f5d2-4732-8d3d-d5a7d7133bc0"

#define NS_IURLPARSER_IID \
  {0x78c5d19f, 0xf5d2, 0x4732, \
    { 0x8d, 0x3d, 0xd5, 0xa7, 0xd7, 0x13, 0x3b, 0xc0 }}

class NS_NO_VTABLE nsIURLParser : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IURLPARSER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIURLParser;

  /* void parseURL (in string spec, in long specLen, out unsigned long schemePos, out long schemeLen, out unsigned long authorityPos, out long authorityLen, out unsigned long pathPos, out long pathLen); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ParseURL(const char * spec, int32_t specLen, uint32_t *schemePos, int32_t *schemeLen, uint32_t *authorityPos, int32_t *authorityLen, uint32_t *pathPos, int32_t *pathLen) = 0;

  /* void parseAuthority (in string authority, in long authorityLen, out unsigned long usernamePos, out long usernameLen, out unsigned long passwordPos, out long passwordLen, out unsigned long hostnamePos, out long hostnameLen, out long port); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ParseAuthority(const char * authority, int32_t authorityLen, uint32_t *usernamePos, int32_t *usernameLen, uint32_t *passwordPos, int32_t *passwordLen, uint32_t *hostnamePos, int32_t *hostnameLen, int32_t *port) = 0;

  /* void parseUserInfo (in string userinfo, in long userinfoLen, out unsigned long usernamePos, out long usernameLen, out unsigned long passwordPos, out long passwordLen); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ParseUserInfo(const char * userinfo, int32_t userinfoLen, uint32_t *usernamePos, int32_t *usernameLen, uint32_t *passwordPos, int32_t *passwordLen) = 0;

  /* void parseServerInfo (in string serverinfo, in long serverinfoLen, out unsigned long hostnamePos, out long hostnameLen, out long port); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ParseServerInfo(const char * serverinfo, int32_t serverinfoLen, uint32_t *hostnamePos, int32_t *hostnameLen, int32_t *port) = 0;

  /* void parsePath (in string path, in long pathLen, out unsigned long filepathPos, out long filepathLen, out unsigned long queryPos, out long queryLen, out unsigned long refPos, out long refLen); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ParsePath(const char * path, int32_t pathLen, uint32_t *filepathPos, int32_t *filepathLen, uint32_t *queryPos, int32_t *queryLen, uint32_t *refPos, int32_t *refLen) = 0;

  /* void parseFilePath (in string filepath, in long filepathLen, out unsigned long directoryPos, out long directoryLen, out unsigned long basenamePos, out long basenameLen, out unsigned long extensionPos, out long extensionLen); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ParseFilePath(const char * filepath, int32_t filepathLen, uint32_t *directoryPos, int32_t *directoryLen, uint32_t *basenamePos, int32_t *basenameLen, uint32_t *extensionPos, int32_t *extensionLen) = 0;

  /* void parseFileName (in string filename, in long filenameLen, out unsigned long basenamePos, out long basenameLen, out unsigned long extensionPos, out long extensionLen); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ParseFileName(const char * filename, int32_t filenameLen, uint32_t *basenamePos, int32_t *basenameLen, uint32_t *extensionPos, int32_t *extensionLen) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIURLParser, NS_IURLPARSER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIURLPARSER \
  NS_IMETHOD ParseURL(const char * spec, int32_t specLen, uint32_t *schemePos, int32_t *schemeLen, uint32_t *authorityPos, int32_t *authorityLen, uint32_t *pathPos, int32_t *pathLen) override; \
  NS_IMETHOD ParseAuthority(const char * authority, int32_t authorityLen, uint32_t *usernamePos, int32_t *usernameLen, uint32_t *passwordPos, int32_t *passwordLen, uint32_t *hostnamePos, int32_t *hostnameLen, int32_t *port) override; \
  NS_IMETHOD ParseUserInfo(const char * userinfo, int32_t userinfoLen, uint32_t *usernamePos, int32_t *usernameLen, uint32_t *passwordPos, int32_t *passwordLen) override; \
  NS_IMETHOD ParseServerInfo(const char * serverinfo, int32_t serverinfoLen, uint32_t *hostnamePos, int32_t *hostnameLen, int32_t *port) override; \
  NS_IMETHOD ParsePath(const char * path, int32_t pathLen, uint32_t *filepathPos, int32_t *filepathLen, uint32_t *queryPos, int32_t *queryLen, uint32_t *refPos, int32_t *refLen) override; \
  NS_IMETHOD ParseFilePath(const char * filepath, int32_t filepathLen, uint32_t *directoryPos, int32_t *directoryLen, uint32_t *basenamePos, int32_t *basenameLen, uint32_t *extensionPos, int32_t *extensionLen) override; \
  NS_IMETHOD ParseFileName(const char * filename, int32_t filenameLen, uint32_t *basenamePos, int32_t *basenameLen, uint32_t *extensionPos, int32_t *extensionLen) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIURLPARSER \
  nsresult ParseURL(const char * spec, int32_t specLen, uint32_t *schemePos, int32_t *schemeLen, uint32_t *authorityPos, int32_t *authorityLen, uint32_t *pathPos, int32_t *pathLen); \
  nsresult ParseAuthority(const char * authority, int32_t authorityLen, uint32_t *usernamePos, int32_t *usernameLen, uint32_t *passwordPos, int32_t *passwordLen, uint32_t *hostnamePos, int32_t *hostnameLen, int32_t *port); \
  nsresult ParseUserInfo(const char * userinfo, int32_t userinfoLen, uint32_t *usernamePos, int32_t *usernameLen, uint32_t *passwordPos, int32_t *passwordLen); \
  nsresult ParseServerInfo(const char * serverinfo, int32_t serverinfoLen, uint32_t *hostnamePos, int32_t *hostnameLen, int32_t *port); \
  nsresult ParsePath(const char * path, int32_t pathLen, uint32_t *filepathPos, int32_t *filepathLen, uint32_t *queryPos, int32_t *queryLen, uint32_t *refPos, int32_t *refLen); \
  nsresult ParseFilePath(const char * filepath, int32_t filepathLen, uint32_t *directoryPos, int32_t *directoryLen, uint32_t *basenamePos, int32_t *basenameLen, uint32_t *extensionPos, int32_t *extensionLen); \
  nsresult ParseFileName(const char * filename, int32_t filenameLen, uint32_t *basenamePos, int32_t *basenameLen, uint32_t *extensionPos, int32_t *extensionLen); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIURLPARSER(_to) \
  NS_IMETHOD ParseURL(const char * spec, int32_t specLen, uint32_t *schemePos, int32_t *schemeLen, uint32_t *authorityPos, int32_t *authorityLen, uint32_t *pathPos, int32_t *pathLen) override { return _to ParseURL(spec, specLen, schemePos, schemeLen, authorityPos, authorityLen, pathPos, pathLen); } \
  NS_IMETHOD ParseAuthority(const char * authority, int32_t authorityLen, uint32_t *usernamePos, int32_t *usernameLen, uint32_t *passwordPos, int32_t *passwordLen, uint32_t *hostnamePos, int32_t *hostnameLen, int32_t *port) override { return _to ParseAuthority(authority, authorityLen, usernamePos, usernameLen, passwordPos, passwordLen, hostnamePos, hostnameLen, port); } \
  NS_IMETHOD ParseUserInfo(const char * userinfo, int32_t userinfoLen, uint32_t *usernamePos, int32_t *usernameLen, uint32_t *passwordPos, int32_t *passwordLen) override { return _to ParseUserInfo(userinfo, userinfoLen, usernamePos, usernameLen, passwordPos, passwordLen); } \
  NS_IMETHOD ParseServerInfo(const char * serverinfo, int32_t serverinfoLen, uint32_t *hostnamePos, int32_t *hostnameLen, int32_t *port) override { return _to ParseServerInfo(serverinfo, serverinfoLen, hostnamePos, hostnameLen, port); } \
  NS_IMETHOD ParsePath(const char * path, int32_t pathLen, uint32_t *filepathPos, int32_t *filepathLen, uint32_t *queryPos, int32_t *queryLen, uint32_t *refPos, int32_t *refLen) override { return _to ParsePath(path, pathLen, filepathPos, filepathLen, queryPos, queryLen, refPos, refLen); } \
  NS_IMETHOD ParseFilePath(const char * filepath, int32_t filepathLen, uint32_t *directoryPos, int32_t *directoryLen, uint32_t *basenamePos, int32_t *basenameLen, uint32_t *extensionPos, int32_t *extensionLen) override { return _to ParseFilePath(filepath, filepathLen, directoryPos, directoryLen, basenamePos, basenameLen, extensionPos, extensionLen); } \
  NS_IMETHOD ParseFileName(const char * filename, int32_t filenameLen, uint32_t *basenamePos, int32_t *basenameLen, uint32_t *extensionPos, int32_t *extensionLen) override { return _to ParseFileName(filename, filenameLen, basenamePos, basenameLen, extensionPos, extensionLen); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIURLPARSER(_to) \
  NS_IMETHOD ParseURL(const char * spec, int32_t specLen, uint32_t *schemePos, int32_t *schemeLen, uint32_t *authorityPos, int32_t *authorityLen, uint32_t *pathPos, int32_t *pathLen) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ParseURL(spec, specLen, schemePos, schemeLen, authorityPos, authorityLen, pathPos, pathLen); } \
  NS_IMETHOD ParseAuthority(const char * authority, int32_t authorityLen, uint32_t *usernamePos, int32_t *usernameLen, uint32_t *passwordPos, int32_t *passwordLen, uint32_t *hostnamePos, int32_t *hostnameLen, int32_t *port) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ParseAuthority(authority, authorityLen, usernamePos, usernameLen, passwordPos, passwordLen, hostnamePos, hostnameLen, port); } \
  NS_IMETHOD ParseUserInfo(const char * userinfo, int32_t userinfoLen, uint32_t *usernamePos, int32_t *usernameLen, uint32_t *passwordPos, int32_t *passwordLen) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ParseUserInfo(userinfo, userinfoLen, usernamePos, usernameLen, passwordPos, passwordLen); } \
  NS_IMETHOD ParseServerInfo(const char * serverinfo, int32_t serverinfoLen, uint32_t *hostnamePos, int32_t *hostnameLen, int32_t *port) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ParseServerInfo(serverinfo, serverinfoLen, hostnamePos, hostnameLen, port); } \
  NS_IMETHOD ParsePath(const char * path, int32_t pathLen, uint32_t *filepathPos, int32_t *filepathLen, uint32_t *queryPos, int32_t *queryLen, uint32_t *refPos, int32_t *refLen) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ParsePath(path, pathLen, filepathPos, filepathLen, queryPos, queryLen, refPos, refLen); } \
  NS_IMETHOD ParseFilePath(const char * filepath, int32_t filepathLen, uint32_t *directoryPos, int32_t *directoryLen, uint32_t *basenamePos, int32_t *basenameLen, uint32_t *extensionPos, int32_t *extensionLen) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ParseFilePath(filepath, filepathLen, directoryPos, directoryLen, basenamePos, basenameLen, extensionPos, extensionLen); } \
  NS_IMETHOD ParseFileName(const char * filename, int32_t filenameLen, uint32_t *basenamePos, int32_t *basenameLen, uint32_t *extensionPos, int32_t *extensionLen) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ParseFileName(filename, filenameLen, basenamePos, basenameLen, extensionPos, extensionLen); } 

// url parser key for use with the category manager
// mapping from scheme to url parser.
#define NS_IURLPARSER_KEY "@mozilla.org/urlparser;1"

#endif /* __gen_nsIURLParser_h__ */
