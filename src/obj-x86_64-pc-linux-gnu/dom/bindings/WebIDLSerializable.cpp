#include "DOMMatrixBinding.h"
#include "DOMPointBinding.h"
#include "DOMQuadBinding.h"
#include "DOMRectBinding.h"
#include "ImageDataBinding.h"
#include "RTCCertificateBinding.h"
#include "StructuredCloneTesterBinding.h"
#include "SubtleCryptoBinding.h"
#include "URLSearchParamsBinding.h"
#include "mozilla/PerfectHash.h"
#include "mozilla/dom/WebIDLSerializable.h"

namespace mozilla {
namespace dom {
struct WebIDLSerializableEntry {
  StructuredCloneTags mTag;
  WebIDLDeserializer mDeserialize;
};

static const WebIDLSerializableEntry sEntries[] = {
  {
    /* mTag */ SCTAG_DOM_CRYPTOKEY,
    /* mDeserialize */ CryptoKey_Binding::Deserialize
  },
  {
    /* mTag */ SCTAG_DOM_DOMMATRIX,
    /* mDeserialize */ DOMMatrix_Binding::Deserialize
  },
  {
    /* mTag */ SCTAG_DOM_DOMMATRIXREADONLY,
    /* mDeserialize */ DOMMatrixReadOnly_Binding::Deserialize
  },
  {
    /* mTag */ SCTAG_DOM_DOMPOINT,
    /* mDeserialize */ DOMPoint_Binding::Deserialize
  },
  {
    /* mTag */ SCTAG_DOM_DOMPOINTREADONLY,
    /* mDeserialize */ DOMPointReadOnly_Binding::Deserialize
  },
  {
    /* mTag */ SCTAG_DOM_DOMQUAD,
    /* mDeserialize */ DOMQuad_Binding::Deserialize
  },
  {
    /* mTag */ SCTAG_DOM_DOMRECT,
    /* mDeserialize */ DOMRect_Binding::Deserialize
  },
  {
    /* mTag */ SCTAG_DOM_DOMRECTREADONLY,
    /* mDeserialize */ DOMRectReadOnly_Binding::Deserialize
  },
  {
    /* mTag */ SCTAG_DOM_IMAGEDATA,
    /* mDeserialize */ ImageData_Binding::Deserialize
  },
  {
    /* mTag */ SCTAG_DOM_RTCCERTIFICATE,
    /* mDeserialize */ RTCCertificate_Binding::Deserialize
  },
  {
    /* mTag */ SCTAG_DOM_STRUCTUREDCLONETESTER,
    /* mDeserialize */ StructuredCloneTester_Binding::Deserialize
  },
  {
    /* mTag */ SCTAG_DOM_URLSEARCHPARAMS,
    /* mDeserialize */ URLSearchParams_Binding::Deserialize
  }
};

WebIDLDeserializer LookupDeserializer(StructuredCloneTags aTag) {
  for (auto& entry : sEntries) {
    if (entry.mTag == aTag) {
      return entry.mDeserialize;
    }
  }
  return nullptr;
}
} // namespace dom
} // namespace mozilla

