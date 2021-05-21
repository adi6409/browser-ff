/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIURI.idl
 */

#ifndef __gen_nsIURI_h__
#define __gen_nsIURI_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "mozilla/AlreadyAddRefed.h"
#include "mozilla/Assertions.h"
#include "mozilla/DebugOnly.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
#include "nsString.h"
#undef GetPort  // XXX Windows!
#undef SetPort  // XXX Windows!
namespace mozilla {
class Encoding;
namespace ipc {
class URIParams;
}  // namespace ipc
}  // namespace mozilla
class nsIURIMutator; /* forward declaration */


/* starting interface:    nsIURI */
#define NS_IURI_IID_STR "92073a54-6d78-4f30-913a-b871813208c6"

#define NS_IURI_IID \
  {0x92073a54, 0x6d78, 0x4f30, \
    { 0x91, 0x3a, 0xb8, 0x71, 0x81, 0x32, 0x08, 0xc6 }}

class nsIURI : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IURI_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIURI;

  /* readonly attribute AUTF8String spec; */
  NS_IMETHOD GetSpec(nsACString& aSpec) = 0;

     // An infallible wrapper for GetSpec() that returns a failure indication
    // string if GetSpec() fails. It is most useful for creating
    // logging/warning/error messages produced for human consumption, and when
    // matching a URI spec against a fixed spec such as about:blank.
    nsCString GetSpecOrDefault()
    {
        nsCString spec;
        nsresult rv = GetSpec(spec);
        if (NS_FAILED(rv)) {
            spec.AssignLiteral("[nsIURI::GetSpec failed]");
        }
        return spec;
    }
  /* readonly attribute AUTF8String prePath; */
  NS_IMETHOD GetPrePath(nsACString& aPrePath) = 0;

  /* readonly attribute ACString scheme; */
  NS_IMETHOD GetScheme(nsACString& aScheme) = 0;

  /* readonly attribute AUTF8String userPass; */
  NS_IMETHOD GetUserPass(nsACString& aUserPass) = 0;

  /* readonly attribute AUTF8String username; */
  NS_IMETHOD GetUsername(nsACString& aUsername) = 0;

  /* readonly attribute AUTF8String password; */
  NS_IMETHOD GetPassword(nsACString& aPassword) = 0;

  /* readonly attribute AUTF8String hostPort; */
  NS_IMETHOD GetHostPort(nsACString& aHostPort) = 0;

  /* readonly attribute AUTF8String host; */
  NS_IMETHOD GetHost(nsACString& aHost) = 0;

  /* readonly attribute long port; */
  NS_IMETHOD GetPort(int32_t *aPort) = 0;

  /* readonly attribute AUTF8String pathQueryRef; */
  NS_IMETHOD GetPathQueryRef(nsACString& aPathQueryRef) = 0;

  /* boolean equals (in nsIURI other); */
  NS_IMETHOD Equals(nsIURI *other, bool *_retval) = 0;

  /* [infallible] boolean schemeIs (in string scheme); */
  NS_IMETHOD SchemeIs(const char * scheme, bool *_retval) = 0;

  inline bool  SchemeIs(const char * scheme)
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = SchemeIs(scheme, &result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }
  /* AUTF8String resolve (in AUTF8String relativePath); */
  NS_IMETHOD Resolve(const nsACString& relativePath, nsACString& _retval) = 0;

  /* readonly attribute ACString asciiSpec; */
  NS_IMETHOD GetAsciiSpec(nsACString& aAsciiSpec) = 0;

  /* readonly attribute ACString asciiHostPort; */
  NS_IMETHOD GetAsciiHostPort(nsACString& aAsciiHostPort) = 0;

  /* readonly attribute ACString asciiHost; */
  NS_IMETHOD GetAsciiHost(nsACString& aAsciiHost) = 0;

  /* readonly attribute AUTF8String ref; */
  NS_IMETHOD GetRef(nsACString& aRef) = 0;

  /* boolean equalsExceptRef (in nsIURI other); */
  NS_IMETHOD EqualsExceptRef(nsIURI *other, bool *_retval) = 0;

  /* readonly attribute AUTF8String specIgnoringRef; */
  NS_IMETHOD GetSpecIgnoringRef(nsACString& aSpecIgnoringRef) = 0;

  /* readonly attribute boolean hasRef; */
  NS_IMETHOD GetHasRef(bool *aHasRef) = 0;

  /* readonly attribute AUTF8String filePath; */
  NS_IMETHOD GetFilePath(nsACString& aFilePath) = 0;

  /* readonly attribute AUTF8String query; */
  NS_IMETHOD GetQuery(nsACString& aQuery) = 0;

  /* readonly attribute AUTF8String displayHost; */
  NS_IMETHOD GetDisplayHost(nsACString& aDisplayHost) = 0;

  /* readonly attribute AUTF8String displayHostPort; */
  NS_IMETHOD GetDisplayHostPort(nsACString& aDisplayHostPort) = 0;

  /* readonly attribute AUTF8String displaySpec; */
  NS_IMETHOD GetDisplaySpec(nsACString& aDisplaySpec) = 0;

  /* readonly attribute AUTF8String displayPrePath; */
  NS_IMETHOD GetDisplayPrePath(nsACString& aDisplayPrePath) = 0;

  /* nsIURIMutator mutate (); */
  NS_IMETHOD Mutate(nsIURIMutator **_retval) = 0;

  /* [noscript,notxpcom] void serialize (in URIParams aParams); */
  NS_IMETHOD_(void) Serialize(mozilla::ipc::URIParams & aParams) = 0;

     // MOZ_DBG support
    friend std::ostream& operator<<(std::ostream& aOut, const nsIURI& aURI) {
      nsIURI* uri = const_cast<nsIURI*>(&aURI);
      return aOut << "nsIURI { " << uri->GetSpecOrDefault() << " }";
    }
    };

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIURI, NS_IURI_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIURI \
  NS_IMETHOD GetSpec(nsACString& aSpec) override; \
  NS_IMETHOD GetPrePath(nsACString& aPrePath) override; \
  NS_IMETHOD GetScheme(nsACString& aScheme) override; \
  NS_IMETHOD GetUserPass(nsACString& aUserPass) override; \
  NS_IMETHOD GetUsername(nsACString& aUsername) override; \
  NS_IMETHOD GetPassword(nsACString& aPassword) override; \
  NS_IMETHOD GetHostPort(nsACString& aHostPort) override; \
  NS_IMETHOD GetHost(nsACString& aHost) override; \
  NS_IMETHOD GetPort(int32_t *aPort) override; \
  NS_IMETHOD GetPathQueryRef(nsACString& aPathQueryRef) override; \
  NS_IMETHOD Equals(nsIURI *other, bool *_retval) override; \
  NS_IMETHOD SchemeIs(const char * scheme, bool *_retval) override; \
  NS_IMETHOD Resolve(const nsACString& relativePath, nsACString& _retval) override; \
  NS_IMETHOD GetAsciiSpec(nsACString& aAsciiSpec) override; \
  NS_IMETHOD GetAsciiHostPort(nsACString& aAsciiHostPort) override; \
  NS_IMETHOD GetAsciiHost(nsACString& aAsciiHost) override; \
  NS_IMETHOD GetRef(nsACString& aRef) override; \
  NS_IMETHOD EqualsExceptRef(nsIURI *other, bool *_retval) override; \
  NS_IMETHOD GetSpecIgnoringRef(nsACString& aSpecIgnoringRef) override; \
  NS_IMETHOD GetHasRef(bool *aHasRef) override; \
  NS_IMETHOD GetFilePath(nsACString& aFilePath) override; \
  NS_IMETHOD GetQuery(nsACString& aQuery) override; \
  NS_IMETHOD GetDisplayHost(nsACString& aDisplayHost) override; \
  NS_IMETHOD GetDisplayHostPort(nsACString& aDisplayHostPort) override; \
  NS_IMETHOD GetDisplaySpec(nsACString& aDisplaySpec) override; \
  NS_IMETHOD GetDisplayPrePath(nsACString& aDisplayPrePath) override; \
  NS_IMETHOD Mutate(nsIURIMutator **_retval) override; \
  NS_IMETHOD_(void) Serialize(mozilla::ipc::URIParams & aParams) override; \

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIURI \
  nsresult GetSpec(nsACString& aSpec); \
  nsresult GetPrePath(nsACString& aPrePath); \
  nsresult GetScheme(nsACString& aScheme); \
  nsresult GetUserPass(nsACString& aUserPass); \
  nsresult GetUsername(nsACString& aUsername); \
  nsresult GetPassword(nsACString& aPassword); \
  nsresult GetHostPort(nsACString& aHostPort); \
  nsresult GetHost(nsACString& aHost); \
  nsresult GetPort(int32_t *aPort); \
  nsresult GetPathQueryRef(nsACString& aPathQueryRef); \
  nsresult Equals(nsIURI *other, bool *_retval); \
  nsresult SchemeIs(const char * scheme, bool *_retval); \
  nsresult Resolve(const nsACString& relativePath, nsACString& _retval); \
  nsresult GetAsciiSpec(nsACString& aAsciiSpec); \
  nsresult GetAsciiHostPort(nsACString& aAsciiHostPort); \
  nsresult GetAsciiHost(nsACString& aAsciiHost); \
  nsresult GetRef(nsACString& aRef); \
  nsresult EqualsExceptRef(nsIURI *other, bool *_retval); \
  nsresult GetSpecIgnoringRef(nsACString& aSpecIgnoringRef); \
  nsresult GetHasRef(bool *aHasRef); \
  nsresult GetFilePath(nsACString& aFilePath); \
  nsresult GetQuery(nsACString& aQuery); \
  nsresult GetDisplayHost(nsACString& aDisplayHost); \
  nsresult GetDisplayHostPort(nsACString& aDisplayHostPort); \
  nsresult GetDisplaySpec(nsACString& aDisplaySpec); \
  nsresult GetDisplayPrePath(nsACString& aDisplayPrePath); \
  nsresult Mutate(nsIURIMutator **_retval); \
  nsresult_(void) Serialize(mozilla::ipc::URIParams & aParams); \

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIURI(_to) \
  NS_IMETHOD GetSpec(nsACString& aSpec) override { return _to GetSpec(aSpec); } \
  NS_IMETHOD GetPrePath(nsACString& aPrePath) override { return _to GetPrePath(aPrePath); } \
  NS_IMETHOD GetScheme(nsACString& aScheme) override { return _to GetScheme(aScheme); } \
  NS_IMETHOD GetUserPass(nsACString& aUserPass) override { return _to GetUserPass(aUserPass); } \
  NS_IMETHOD GetUsername(nsACString& aUsername) override { return _to GetUsername(aUsername); } \
  NS_IMETHOD GetPassword(nsACString& aPassword) override { return _to GetPassword(aPassword); } \
  NS_IMETHOD GetHostPort(nsACString& aHostPort) override { return _to GetHostPort(aHostPort); } \
  NS_IMETHOD GetHost(nsACString& aHost) override { return _to GetHost(aHost); } \
  NS_IMETHOD GetPort(int32_t *aPort) override { return _to GetPort(aPort); } \
  NS_IMETHOD GetPathQueryRef(nsACString& aPathQueryRef) override { return _to GetPathQueryRef(aPathQueryRef); } \
  NS_IMETHOD Equals(nsIURI *other, bool *_retval) override { return _to Equals(other, _retval); } \
  NS_IMETHOD SchemeIs(const char * scheme, bool *_retval) override { return _to SchemeIs(scheme, _retval); } \
  NS_IMETHOD Resolve(const nsACString& relativePath, nsACString& _retval) override { return _to Resolve(relativePath, _retval); } \
  NS_IMETHOD GetAsciiSpec(nsACString& aAsciiSpec) override { return _to GetAsciiSpec(aAsciiSpec); } \
  NS_IMETHOD GetAsciiHostPort(nsACString& aAsciiHostPort) override { return _to GetAsciiHostPort(aAsciiHostPort); } \
  NS_IMETHOD GetAsciiHost(nsACString& aAsciiHost) override { return _to GetAsciiHost(aAsciiHost); } \
  NS_IMETHOD GetRef(nsACString& aRef) override { return _to GetRef(aRef); } \
  NS_IMETHOD EqualsExceptRef(nsIURI *other, bool *_retval) override { return _to EqualsExceptRef(other, _retval); } \
  NS_IMETHOD GetSpecIgnoringRef(nsACString& aSpecIgnoringRef) override { return _to GetSpecIgnoringRef(aSpecIgnoringRef); } \
  NS_IMETHOD GetHasRef(bool *aHasRef) override { return _to GetHasRef(aHasRef); } \
  NS_IMETHOD GetFilePath(nsACString& aFilePath) override { return _to GetFilePath(aFilePath); } \
  NS_IMETHOD GetQuery(nsACString& aQuery) override { return _to GetQuery(aQuery); } \
  NS_IMETHOD GetDisplayHost(nsACString& aDisplayHost) override { return _to GetDisplayHost(aDisplayHost); } \
  NS_IMETHOD GetDisplayHostPort(nsACString& aDisplayHostPort) override { return _to GetDisplayHostPort(aDisplayHostPort); } \
  NS_IMETHOD GetDisplaySpec(nsACString& aDisplaySpec) override { return _to GetDisplaySpec(aDisplaySpec); } \
  NS_IMETHOD GetDisplayPrePath(nsACString& aDisplayPrePath) override { return _to GetDisplayPrePath(aDisplayPrePath); } \
  NS_IMETHOD Mutate(nsIURIMutator **_retval) override { return _to Mutate(_retval); } \
  NS_IMETHOD_(void) Serialize(mozilla::ipc::URIParams & aParams) override { return _to Serialize(aParams); } \

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIURI(_to) \
  NS_IMETHOD GetSpec(nsACString& aSpec) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSpec(aSpec); } \
  NS_IMETHOD GetPrePath(nsACString& aPrePath) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPrePath(aPrePath); } \
  NS_IMETHOD GetScheme(nsACString& aScheme) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetScheme(aScheme); } \
  NS_IMETHOD GetUserPass(nsACString& aUserPass) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetUserPass(aUserPass); } \
  NS_IMETHOD GetUsername(nsACString& aUsername) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetUsername(aUsername); } \
  NS_IMETHOD GetPassword(nsACString& aPassword) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPassword(aPassword); } \
  NS_IMETHOD GetHostPort(nsACString& aHostPort) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHostPort(aHostPort); } \
  NS_IMETHOD GetHost(nsACString& aHost) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHost(aHost); } \
  NS_IMETHOD GetPort(int32_t *aPort) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPort(aPort); } \
  NS_IMETHOD GetPathQueryRef(nsACString& aPathQueryRef) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPathQueryRef(aPathQueryRef); } \
  NS_IMETHOD Equals(nsIURI *other, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Equals(other, _retval); } \
  NS_IMETHOD SchemeIs(const char * scheme, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SchemeIs(scheme, _retval); } \
  NS_IMETHOD Resolve(const nsACString& relativePath, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Resolve(relativePath, _retval); } \
  NS_IMETHOD GetAsciiSpec(nsACString& aAsciiSpec) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAsciiSpec(aAsciiSpec); } \
  NS_IMETHOD GetAsciiHostPort(nsACString& aAsciiHostPort) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAsciiHostPort(aAsciiHostPort); } \
  NS_IMETHOD GetAsciiHost(nsACString& aAsciiHost) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAsciiHost(aAsciiHost); } \
  NS_IMETHOD GetRef(nsACString& aRef) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRef(aRef); } \
  NS_IMETHOD EqualsExceptRef(nsIURI *other, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EqualsExceptRef(other, _retval); } \
  NS_IMETHOD GetSpecIgnoringRef(nsACString& aSpecIgnoringRef) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSpecIgnoringRef(aSpecIgnoringRef); } \
  NS_IMETHOD GetHasRef(bool *aHasRef) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHasRef(aHasRef); } \
  NS_IMETHOD GetFilePath(nsACString& aFilePath) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFilePath(aFilePath); } \
  NS_IMETHOD GetQuery(nsACString& aQuery) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetQuery(aQuery); } \
  NS_IMETHOD GetDisplayHost(nsACString& aDisplayHost) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDisplayHost(aDisplayHost); } \
  NS_IMETHOD GetDisplayHostPort(nsACString& aDisplayHostPort) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDisplayHostPort(aDisplayHostPort); } \
  NS_IMETHOD GetDisplaySpec(nsACString& aDisplaySpec) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDisplaySpec(aDisplaySpec); } \
  NS_IMETHOD GetDisplayPrePath(nsACString& aDisplayPrePath) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDisplayPrePath(aDisplayPrePath); } \
  NS_IMETHOD Mutate(nsIURIMutator **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Mutate(_retval); } \
  NS_IMETHOD_(void) Serialize(mozilla::ipc::URIParams & aParams) override; \


#endif /* __gen_nsIURI_h__ */
