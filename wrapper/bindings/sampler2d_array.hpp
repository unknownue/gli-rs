/// @brief Include to sample 2d array textures.
/// @file gli/sampler2d_array.hpp

#pragma once

#include "sampler.hpp"
#include "texture2d_array.hpp"
#include "mipmaps_compute.hpp"
#include "convert_func.hpp"
#include "clear.hpp"

namespace gli
{
	/// 2d array texture sampler
	/// @tparam T Sampler can fetch, write and interpret any texture format but will expose and process the data through type T conversions.
	/// @tparam Q Precision in term of ULPs
	template <typename T, qualifier P = defaultp>
	class sampler2d_array : public sampler
	{
	private:
		typedef typename detail::interpolate<T>::type interpolate_type;

	public:
		typedef texture2d_array texture_type;
		typedef typename texture_type::size_type size_type;
		typedef typename texture_type::extent_type extent_type;
		typedef interpolate_type level_type;
		typedef vec<2, interpolate_type, P> normalized_type;
		typedef vec<4, T, P> texel_type;

		sampler2d_array(texture_type const& Texture, wrap Wrap, filter Mip = FILTER_NEAREST, filter Min = FILTER_NEAREST, texel_type const& BorderColor = texel_type(0, 0, 0, 1));

        void set_border_color(texel_type BorderColor) {
            this->BorderColor = BorderColor;
        }

		/// Access the sampler texture object
		texture_type const& operator()() const;

		/// Fetch a texel from the sampler texture
		texel_type texel_fetch(extent_type const& TexelCoord, size_type layer, size_type Level) const;

		/// Write a texel in the sampler texture
		void texel_write(extent_type const& TexelCoord, size_type layer, size_type Level, texel_type const& Texel);

		/// Clear the sampler texture with a uniform texel
		void clear(texel_type const& Texel);

		/// Sample the sampler texture at a specific level
		texel_type texture_lod(normalized_type const& SampleCoord, size_type layer, level_type Level) const;

		/// Generate all the mipmaps of the sampler texture from the texture base level
		void generate_mipmaps(filter Minification);

		/// Generate the mipmaps of the sampler texture from the texture base level to the texture max level included
		void generate_mipmaps(size_type BaseLayer, size_type MaxLayer, size_type BaseLevel, size_type MaxLevel, filter Minification);

	private:
		typedef typename detail::convert<texture_type, T, P>::func convert_type;
		typedef typename detail::convert<texture_type, T, P>::fetchFunc fetch_type;
		typedef typename detail::convert<texture_type, T, P>::writeFunc write_type;
		typedef typename detail::filterBase<detail::DIMENSION_2D, texture_type, interpolate_type, normalized_type, fetch_type, texel_type>::filterFunc filter_type;

		texture_type Texture;
		convert_type Convert;
		texel_type BorderColor;
		filter_type Filter;
	};

	typedef sampler2d_array<float> fsampler2DArray;
	typedef sampler2d_array<double> dsampler2DArray;
	typedef sampler2d_array<unsigned int> usampler2DArray;
	typedef sampler2d_array<int> isampler2DArray;

}//namespace gli


extern "C" {
    
    namespace bindings {
    
        namespace FSampler2DArray {

#ifndef _WIN32
            gli::fsampler2DArray fsampler2darray_new(const gli::texture2d_array & Texture, gli::wrap Wrap, gli::filter Mip, gli::filter Min) {
                return gli::fsampler2DArray(Texture, Wrap, Mip, Min);
            }
#endif

            void fsampler2darray_set_border_color(gli::fsampler2DArray & Sampler, TexelType4F BorderColor) {
                Sampler.set_border_color(gli::tex4FToVec4(BorderColor));
            }
    
            void fsampler2darray_clear(gli::fsampler2DArray & Sampler, TexelType4F Texel) {
                Sampler.clear(gli::tex4FToVec4(Texel));
            }
    
            TexelType4F fsampler2darray_texel_fetch(const gli::fsampler2DArray & Sampler, gli::fsampler2DArray::extent_type TexelCoord, gli::texture::size_type Layer, gli::texture::size_type Level) {
                gli::vec4 raw = Sampler.texel_fetch(TexelCoord, Layer, Level);
                return vec4ToTex4F(raw);
            }
    
            void fsampler2darray_texel_write(gli::fsampler2DArray & Sampler, gli::fsampler2DArray::extent_type TexelCoord, gli::texture::size_type Layer, gli::texture::size_type Level, TexelType4F Texel) {
                Sampler.texel_write(TexelCoord, Layer, Level, gli::tex4FToVec4(Texel));
            }
    
            TexelType4F fsampler2darray_texel_lod(const gli::fsampler2DArray & Sampler, const float SampleCoord[2], gli::texture::size_type Layer, float Level) {
                gli::vec4 raw = Sampler.texture_lod(gli::fsampler2DArray::normalized_type(SampleCoord[0], SampleCoord[1]), Layer, Level);
                return vec4ToTex4F(raw);
            }
    
            const gli::texture2d_array & fsampler2darray_target_texture(const gli::fsampler2DArray & Sampler) {
                return Sampler.operator()();
            }
    
            void fsampler2darray_generate_mipmaps1(gli::fsampler2DArray & Sampler, gli::filter Minification) {
                Sampler.generate_mipmaps(Minification);
            }
    
            void fsampler2darray_generate_mipmaps2(gli::fsampler2DArray & Sampler, gli::texture::size_type BaseLayer, gli::texture::size_type MaxLayer, gli::texture::size_type BaseLevel, gli::texture::size_type MaxLevel, gli::filter Minification) {
                Sampler.generate_mipmaps(BaseLayer, MaxLayer, BaseLevel, MaxLevel, Minification);
            }
        }
    }
}


#ifdef GLI_IMPLEMENTATION
#include "sampler2d_array.inl"
#endif
