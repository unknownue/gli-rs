
#include <glm/gtc/epsilon.hpp>


extern "C" {

    namespace bindings {

        struct TexelType4F {
            float content[4];
        };

        TexelType4F vec4ToTex4F(gli::vec4 raw) {
            TexelType4F value;
            value.content[0] = raw[0];
            value.content[1] = raw[1];
            value.content[2] = raw[2];
            value.content[3] = raw[3];
            return value;
        }
    }
}

namespace gli {

    gli::vec4 tex4FToVec4(bindings::TexelType4F raw) {
        return gli::vec4(raw.content[0], raw.content[1], raw.content[2], raw.content[3]);
    }
}
