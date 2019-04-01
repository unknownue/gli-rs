/// @brief Include to sample 2d textures.
/// @file gli/sampler2d.hpp

#pragma once

#include "sampler.hpp"
#include "texture2d.hpp"
#include "mipmaps_compute.hpp"
#include "convert_func.hpp"
#include "clear.hpp"

namespace gli
{
	/// 2d texture sampler
	/// @tparam T Sampler can fetch, write and interpret any texture format but will expose and process the data through type T conversions.
	/// @tparam P Precision in term of ULPs
	template <typename T, qualifier P = defaultp>
	class sampler2d : public sampler
	{
	private:
		typedef typename detail::interpolate<T>::type interpolate_type;

	public:
		typedef texture2d texture_type;
		typedef typename texture_type::size_type size_type;
		typedef typename texture_type::extent_type extent_type;
		typedef interpolate_type level_type;
		typedef vec<2, interpolate_type, P> normalized_type;
		typedef vec<4, T, P> texel_type;

		sampler2d(texture_type const& Texture, wrap Wrap, filter Mip = FILTER_NEAREST, filter Min = FILTER_NEAREST, texel_type const& BorderColor = texel_type(0, 0, 0, 1));

        void set_border_color(texel_type BorderColor) {
            this->BorderColor = BorderColor;
        }

		/// Access the sampler texture object
		texture_type const& operator()() const;

		/// Fetch a texel from the sampler texture
		texel_type texel_fetch(extent_type const& TexelCoord, size_type const& Level) const;

		/// Write a texel in the sampler texture
		void texel_write(extent_type const& TexelCoord, size_type const& Level, texel_type const& Texel);

		/// Clear the sampler texture with a uniform texel
		void clear(texel_type const& Texel);

		/// Sample a texture at a specific level
		texel_type texture_lod(normalized_type const& SampleCoord, level_type Level) const;

		/// Sample a texture using specified gradiants
		texel_type texture_grad(normalized_type const& SampleCoord, normalized_type const& dPdx, normalized_type const& dPdy) const;

		/// Generate all the mipmaps of the sampler texture from the texture base level
		void generate_mipmaps(filter Minification);

		/// Generate the mipmaps of the sampler texture from the texture base level to the texture max level included
		void generate_mipmaps(size_type BaseLevel, size_type MaxLevel, filter Minification);

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

	typedef sampler2d<float> fsampler2D;
	typedef sampler2d<double> dsampler2D;
	typedef sampler2d<unsigned int> usampler2D;
	typedef sampler2d<int> isampler2D;

}//namespace gli


extern "C" {
    
    namespace bindings {
    
        namespace FSampler2D {
    
            gli::fsampler2D fsampler2d_new(const gli::texture2d & Texture, gli::wrap Wrap, gli::filter Mip, gli::filter Min) {
                return gli::fsampler2D(Texture, Wrap, Mip, Min);
            }
    
            void fsampler2d_set_border_color(gli::fsampler2D & Sampler, gli::fsampler2D::texel_type BorderColor) {
                Sampler.set_border_color(BorderColor);
            }
    
            void fsampler2d_clear(gli::fsampler2D & Sampler, gli::fsampler2D::texel_type Texel) {
                Sampler.clear(Texel);
            }
    
            gli::fsampler2D::texel_type fsampler2d_texel_fetch(const gli::fsampler2D & Sampler, gli::fsampler2D::extent_type TexelCoord, gli::texture::size_type Level) {
                return Sampler.texel_fetch(TexelCoord, Level);
            }
    
            void fsampler2d_texel_write(gli::fsampler2D & Sampler, gli::fsampler2D::extent_type TexelCoord, gli::texture::size_type Level, gli::fsampler2D::texel_type Texel) {
                return Sampler.texel_write(TexelCoord, Level, Texel);
            }
    
            gli::fsampler2D::texel_type fsampler2d_texel_lod(const gli::fsampler2D & Sampler, gli::vec<2, float, (glm::qualifier)0U> SampleCoord, gli::texture::size_type Level) {
                return Sampler.texture_lod(SampleCoord, Level);
            }
    
            const gli::texture2d & fsampler2d_target_texture(const gli::fsampler2D & Sampler) {
                return Sampler.operator()();
            }
    
            void fsampler2d_generate_mipmaps1(gli::fsampler2D & Sampler, gli::filter Minification) {
                Sampler.generate_mipmaps(Minification);
            }
    
            void fsampler2d_generate_mipmaps2(gli::fsampler2D & Sampler, gli::texture::size_type BaseLevel, gli::texture::size_type  MaxLevel, gli::filter Minification) {
                Sampler.generate_mipmaps(BaseLevel, MaxLevel, Minification);
            }
        }
    }
}

#ifdef GLI_IMPLEMENTATION
#include "sampler2d.inl"
#endif
