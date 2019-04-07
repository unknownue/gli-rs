/// @brief Include to sample 1d textures.
/// @file gli/sampler1d.hpp

#pragma once

#include "sampler.hpp"
#include "texture1d.hpp"
#include "mipmaps_compute.hpp"
#include "convert_func.hpp"
#include "clear.hpp"

namespace gli
{
	/// 1d texture sampler
	/// @tparam T Sampler can fetch, write and interpret any texture format but will expose and process the data through type T conversions.
	/// @tparam P Precision in term of ULPs
	template <typename T, qualifier P = defaultp>
	class sampler1d : public sampler
	{
	private:
		typedef typename detail::interpolate<T>::type interpolate_type;

	public:
		typedef texture1d texture_type;
		typedef typename texture_type::size_type size_type;
		typedef typename texture_type::extent_type extent_type;
		typedef interpolate_type level_type;
		typedef vec<1, interpolate_type, P> normalized_type;
		typedef vec<4, T, P> texel_type;

		sampler1d(texture_type const& Texture, wrap Wrap, filter Mip = FILTER_NEAREST, filter Min = FILTER_NEAREST, texel_type const& BorderColor = texel_type(0, 0, 0, 1));

		/// Access the sampler texture object
		texture_type const& operator()() const;

		void set_border_color(texel_type BorderColor) {
		    this->BorderColor = BorderColor;
		}

		/// Fetch a texel from the sampler texture
		texel_type texel_fetch(extent_type const& TexelCoord, size_type const& Level) const;

		/// Write a texel in the sampler texture
		void texel_write(extent_type const& TexelCoord, size_type const& Level, texel_type const& Texel);

		/// Clear the sampler texture with a uniform texel
		void clear(texel_type const& Texel);

		/// Sample the sampler texture at a specific level
		texel_type texture_lod(normalized_type const& SampleCoord, level_type Level) const;

		/// Generate all the mipmaps of the sampler texture from the texture base level
		void generate_mipmaps(filter Minification);

		/// Generate the mipmaps of the sampler texture from the texture base level to the texture max level included
		void generate_mipmaps(size_type BaseLevel, size_type MaxLevel, filter Minification);

	private:
		typedef typename detail::convert<texture_type, T, P>::func convert_type;
		typedef typename detail::convert<texture_type, T, P>::fetchFunc fetch_type;
		typedef typename detail::convert<texture_type, T, P>::writeFunc write_type;
		typedef typename detail::filterBase<detail::DIMENSION_1D, texture_type, interpolate_type, normalized_type, fetch_type, texel_type>::filterFunc filter_type;

		texture_type Texture;
		convert_type Convert;
		texel_type BorderColor;
		filter_type Filter;
	};

	typedef sampler1d<float> fsampler1D;
	typedef sampler1d<double> dsampler1D;
	typedef sampler1d<unsigned int> usampler1D;
	typedef sampler1d<int> isampler1D;
}//namespace gli

extern "C" {

    namespace bindings {

        namespace FSampler1D {
            
            gli::fsampler1D fsampler1d_new(const gli::texture1d & Texture, gli::wrap Wrap, gli::filter Mip, gli::filter Min) {
                return gli::fsampler1D(Texture, Wrap, Mip, Min);
            }

            void fsampler1d_set_border_color(gli::fsampler1D & Sampler, TexelType4F BorderColor) {
                Sampler.set_border_color(gli::tex4FToVec4(BorderColor));
            }

            void fsampler1d_clear(gli::fsampler1D & Sampler, TexelType4F Texel) {
                Sampler.clear(gli::tex4FToVec4(Texel));
            }

            TexelType4F fsampler1d_texel_fetch(const gli::fsampler1D & Sampler, gli::fsampler1D::extent_type TexelCoord, gli::texture::size_type Level) {
                gli::vec4 raw = Sampler.texel_fetch(TexelCoord, Level);
                return vec4ToTex4F(raw);
            }

            void fsampler1d_texel_write(gli::fsampler1D & Sampler, gli::fsampler1D::extent_type TexelCoord, gli::texture::size_type Level, TexelType4F Texel) {
                Sampler.texel_write(TexelCoord, Level, gli::tex4FToVec4(Texel));
            }

            TexelType4F fsampler1d_texel_lod(const gli::fsampler1D & Sampler, float SampleCoord, float Level) {
                gli::vec4 raw = Sampler.texture_lod(gli::fsampler1D::normalized_type(SampleCoord), Level);
                return vec4ToTex4F(raw);
            }

            const gli::texture1d & fsampler1d_target_texture(const gli::fsampler1D & Sampler) {
                return Sampler.operator()();
            }

            void fsampler1d_generate_mipmaps1(gli::fsampler1D & Sampler, gli::filter Minification) {
                Sampler.generate_mipmaps(Minification);
            }

            void fsampler1d_generate_mipmaps2(gli::fsampler1D & Sampler, gli::texture::size_type BaseLevel, gli::texture::size_type  MaxLevel, gli::filter Minification) {
                Sampler.generate_mipmaps(BaseLevel, MaxLevel, Minification);
            }
        }

//        namespace DSampler1D {
//
//            gli::dsampler1D dsampler1D_new(const gli::texture1d & Texture, gli::wrap Wrap, gli::filter Mip, gli::filter Min) {
//                return gli::dsampler1D(Texture, Wrap, Mip, Min);
//            }
//
//            void dsampler1D_set_border_color(gli::dsampler1D & Sampler, gli::dsampler1D::texel_type BorderColor) {
//                Sampler.set_border_color(BorderColor);
//            }
//
//            void dsampler1D_clear(gli::dsampler1D & Sampler, gli::dsampler1D::texel_type & Texel) {
//                Sampler.clear(Texel);
//            }
//
//            void dsampler1D_generate_mipmaps1(gli::dsampler1D & Sampler, gli::filter Minification) {
//                Sampler.generate_mipmaps(Minification);
//            }
//
//            void dsampler1D_generate_mipmaps2(gli::dsampler1D & Sampler, gli::texture::size_type BaseLevel, gli::texture::size_type MaxLevel, gli::filter Minification) {
//                Sampler.generate_mipmaps(BaseLevel, MaxLevel, Minification);
//            }
//
//            gli::dsampler1D::texel_type dsampler1D_texel_fetch(const gli::dsampler1D & Sampler, const gli::dsampler1D::extent_type & TexelCoord, const gli::texture::size_type & Level) {
//                return Sampler.texel_fetch(TexelCoord, Level);
//            }
//
//            void dsampler1D_texel_write(gli::dsampler1D & Sampler, const gli::dsampler1D::extent_type & TexelCoord, const gli::texture::size_type & Level, const gli::dsampler1D::texel_type & Texel) {
//                return Sampler.texel_write(TexelCoord, Level, Texel);
//            }
//
//            gli::dsampler1D::texel_type dsampler1D_texel_lod(const gli::dsampler1D & Sampler, const gli::vec<1, double, (glm::qualifier)0U> SampleCoord, const gli::texture::size_type & Level) {
//                return Sampler.texture_lod(SampleCoord, Level);
//            }
//
//            const gli::texture1d & dsampler1D_target_texture(const gli::dsampler1D & Sampler) {
//                return Sampler.operator()();
//            }
//        }
//
//        namespace USampler1D {
//
//            gli::usampler1D usampler1d_new(const gli::texture1d & Texture, gli::wrap Wrap, gli::filter Mip, gli::filter Min) {
//                return gli::usampler1D(Texture, Wrap, Mip, Min, gli::usampler1D::texel_type(0, 0, 0, 1));
//            }
//
//            void usampler1D_set_border_color(gli::usampler1D & Sampler, gli::usampler1D::texel_type BorderColor) {
//                Sampler.set_border_color(BorderColor);
//            }
//
//            void usampler1D_clear(gli::usampler1D & Sampler, gli::usampler1D::texel_type & Texel) {
//                Sampler.clear(Texel);
//            }
//
//            void usampler1D_generate_mipmaps1(gli::usampler1D & Sampler, gli::filter Minification) {
//                Sampler.generate_mipmaps(Minification);
//            }
//
//            void usampler1D_generate_mipmaps2(gli::usampler1D & Sampler, gli::texture::size_type BaseLevel, gli::texture::size_type MaxLevel, gli::filter Minification) {
//                Sampler.generate_mipmaps(BaseLevel, MaxLevel, Minification);
//            }
//
//            gli::usampler1D::texel_type usampler1D_texel_fetch(const gli::usampler1D & Sampler, const gli::usampler1D::extent_type & TexelCoord, const gli::texture::size_type & Level) {
//                return Sampler.texel_fetch(TexelCoord, Level);
//            }
//
//            void usampler1D_texel_write(gli::usampler1D & Sampler, const gli::usampler1D::extent_type & TexelCoord, const gli::texture::size_type & Level, const gli::usampler1D::texel_type & Texel) {
//                return Sampler.texel_write(TexelCoord, Level, Texel);
//            }
//
//            gli::usampler1D::texel_type usampler1D_texel_lod(const gli::usampler1D & Sampler, const gli::vec<1, float, (glm::qualifier)0U> SampleCoord, const gli::texture::size_type & Level) {
//                return Sampler.texture_lod(SampleCoord, Level);
//            }
//
//            const gli::texture1d & usampler1D_target_texture(const gli::usampler1D & Sampler) {
//                return Sampler.operator()();
//            }
//        }
    }
}

#ifdef GLI_IMPLEMENTATION
#include "sampler1d.inl"
#endif
