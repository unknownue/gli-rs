/// @brief Include to use 1d textures.
/// @file gli/texture1d.hpp

#pragma once

#include "texture.hpp"
#include "image.hpp"

namespace gli {

    /// 1d texture
    class texture1d : public texture {
    public:
        typedef extent1d extent_type;

        /// Create an empty texture 1D
        texture1d();

        /// Create a texture1d and allocate a new storage_linear
        texture1d(
            format_type Format,
            extent_type const& Extent,
            size_type Levels,
            swizzles_type const& Swizzles = swizzles_type(SWIZZLE_RED, SWIZZLE_GREEN, SWIZZLE_BLUE, SWIZZLE_ALPHA));

        /// Create a texture1d and allocate a new storage_linear with a complete mipmap chain
        texture1d(
            format_type Format,
            extent_type const& Extent,
            swizzles_type const& Swizzles = swizzles_type(SWIZZLE_RED, SWIZZLE_GREEN, SWIZZLE_BLUE, SWIZZLE_ALPHA));

        /// Create a texture1d view with an existing storage_linear
        explicit texture1d(texture const& Texture);

        /// Create a texture1d view with an existing storage_linear
        texture1d(
            texture const& Texture,
            format_type Format,
            size_type BaseLayer, size_type MaxLayer,
            size_type BaseFace, size_type MaxFace,
            size_type BaseLevel, size_type MaxLevel,
            swizzles_type const& Swizzles = swizzles_type(SWIZZLE_RED, SWIZZLE_GREEN, SWIZZLE_BLUE, SWIZZLE_ALPHA));

        /// Create a texture1d view, reference a subset of an existing texture1d instance
        texture1d(
            texture1d const& Texture,
            size_type BaseLevel, size_type MaxLevel);

        /// Create a view of the image identified by Level in the mipmap chain of the texture
        image operator[](size_type Level) const;

        /// Return the width of a texture instance
        extent_type extent(size_type Level = 0) const;

        /// Fetch a texel from a texture. The texture format must be uncompressed.
        template <typename gen_type>
        gen_type load(extent_type const& TexelCoord, size_type Level) const;

        /// Write a texel to a texture. The texture format must be uncompressed.
        template <typename gen_type>
        void store(extent_type const& TexelCoord, size_type Level, gen_type const& Texel);
    };
}//namespace gli

extern "C" {

    namespace bindings {

        namespace Texture1D {

            using gli::texture1d;

            texture1d tex1d_new_empty() {
                return texture1d();
            }

            texture1d tex1d_new_(texture1d::format_type format, texture1d::extent_type extent, texture1d::size_type levels) {
                return texture1d(format, extent, levels);
            }

            texture1d tex1d_new_with_mipmap_chain(texture1d::format_type format, texture1d::extent_type extent) {
                return texture1d(format, extent);
            }

            texture1d tex1d_share_from(const texture1d & tex) {
                return texture1d(tex);
            }

            texture1d tex1d_share_from_detail(
                const texture1d & tex,
                texture1d::format_type format,
                texture1d::size_type base_layer, texture1d::size_type max_layer,
                texture1d::size_type base_face,  texture1d::size_type  max_face,
                texture1d::size_type base_level, texture1d::size_type max_level) {

                return texture1d(tex, format, base_layer, max_level, base_face, max_face, base_level, max_level);
            }

            texture1d tex1d_share_from_subset(const texture1d & tex, texture1d::size_type base_level, texture1d::size_type max_level) {

                return texture1d(tex, base_level, max_level);
            }

            texture1d::extent_type tex1d_extent(const texture1d & tex, texture1d::size_type level = 0) {
                return tex.extent(level);
            }
        }
    }
}

#ifdef GLI_IMPLEMENTATION
#include "texture1d.inl"
#endif
