/// @brief Include to use 2d textures.
/// @file gli/texture2d.hpp

#pragma once

#include "texture.hpp"
#include "image.hpp"

namespace gli {
    /// 2d texture
    class texture2d : public texture {
    public:
        typedef extent2d extent_type;

        /// Create an empty texture 2D.
        texture2d();

        /// Create a texture2d and allocate a new storage_linear.
        texture2d(
            format_type Format,
            extent_type const& Extent,
            size_type Levels,
            swizzles_type const& Swizzles = swizzles_type(SWIZZLE_RED, SWIZZLE_GREEN, SWIZZLE_BLUE, SWIZZLE_ALPHA));

        /// Create a texture2d and allocate a new storage_linear with a complete mipmap chain.
        texture2d(
            format_type Format,
            extent_type const& Extent,
            swizzles_type const& Swizzles = swizzles_type(SWIZZLE_RED, SWIZZLE_GREEN, SWIZZLE_BLUE, SWIZZLE_ALPHA));

        /// Create a texture2d view with an existing storage_linear.
        explicit texture2d(texture const& Texture);

        /// Create a texture2d view with an existing storage_linear.
        texture2d(
            texture const& Texture,
            format_type Format,
            size_type BaseLayer, size_type MaxLayer,
            size_type BaseFace, size_type MaxFace,
            size_type BaseLevel, size_type MaxLevel,
            swizzles_type const& Swizzles = swizzles_type(SWIZZLE_RED, SWIZZLE_GREEN, SWIZZLE_BLUE, SWIZZLE_ALPHA));

        /// Create a texture2d view, reference a subset of an existing texture2d instance.
        texture2d(
            texture2d const& Texture,
            size_type BaseLevel, size_type MaxLevel);

        /// Create a view of the image identified by Level in the mipmap chain of the texture.
        image operator[](size_type Level) const;

        /// Return the dimensions of a texture instance: width and height.
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

        namespace Texture2D {

            using gli::texture2d;

            texture2d tex2d_new_empty() {
                return texture2d();
            }

            texture2d tex2d_new_(texture2d::format_type format, texture2d::extent_type extent, texture2d::size_type levels) {
                return texture2d(format, extent, levels);
            }

            texture2d tex2d_new_with_mipmap_chain(texture2d::format_type format, texture2d::extent_type extent) {
                return texture2d(format, extent);
            }

            texture2d tex2d_share_from(const gli::texture & tex) {
                return texture2d(tex);
            }

            texture2d tex2d_share_from_detail(
                const gli::texture & tex,
                texture2d::format_type format,
                texture2d::size_type base_layer, texture2d::size_type max_layer,
                texture2d::size_type base_face,  texture2d::size_type  max_face,
                texture2d::size_type base_level, texture2d::size_type max_level) {

                return texture2d(tex, format, base_layer, max_level, base_face, max_face, base_level, max_level);
            }

            texture2d tex2d_share_from_subset(const texture2d & tex, texture2d::size_type base_level, texture2d::size_type max_level) {

                return texture2d(tex, base_level, max_level);
            }

            texture2d::extent_type tex2d_extent(const texture2d & tex, texture2d::size_type level = 0) {
                return tex.extent(level);
            }
        }
    }
}

#ifdef GLI_IMPLEMENTATION
#include "texture2d.inl"
#endif
