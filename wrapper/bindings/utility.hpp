
#include <glm/gtc/epsilon.hpp>

extern "C" {

    namespace bindings {

        struct TexelType4F {
            float content[4];

            TexelType4F(gli::vec4 raw) {
                this->content[0] = raw[0];
                this->content[1] = raw[1];
                this->content[2] = raw[2];
                this->content[3] = raw[3];
            }

            gli::vec4 into_raw() const {
                return gli::vec4(this->content[0], this->content[1], this->content[2], this->content[3]);
            }
        };
    }
}
