/// @brief Include to load DDS, KTX or KMG textures from files or memory.
/// @file gli/load.hpp

#pragma once

#include "texture.hpp"
#include "gl.hpp"
#include "dx.hpp"

namespace gli {

    /// Loads a texture storage_linear from file. Returns an empty storage_linear in case of failure.
    ///
    /// @param Path Path of the file to open including filaname and filename extension
    texture load(char const* Path);

    /// Loads a texture storage_linear from file. Returns an empty storage_linear in case of failure.
    ///
    /// @param Path Path of the file to open including filaname and filename extension
    texture load(std::string const& Path);

    /// Loads a texture storage_linear from memory. Returns an empty storage_linear in case of failure.
    ///
    /// @param Data Data of a texture
    /// @param Size Size of the data
    texture load(char const* Data, std::size_t Size);

    /// Loads a texture storage_linear from DDS file. Returns an empty storage_linear in case of failure.
    ///
    /// @param Path Path of the file to open including filaname and filename extension
    texture load_dds(char const* Path);

    /// Loads a texture storage_linear from DDS file. Returns an empty storage_linear in case of failure.
    ///
    /// @param Path Path of the file to open including filaname and filename extension
    texture load_dds(std::string const& Path);

    /// Loads a texture storage_linear from DDS memory. Returns an empty storage_linear in case of failure.
    ///
    /// @param Data Pointer to the beginning of the texture container data to read
    /// @param Size Size of texture container Data to read
    texture load_dds(char const* Data, std::size_t Size);

    /// Loads a texture storage_linear from KMG (Khronos Image) file. Returns an empty storage_linear in case of failure.
    ///
    /// @param Path Path of the file to open including filaname and filename extension
    texture load_kmg(char const* Path);

    /// Loads a texture storage_linear from KMG (Khronos Image) file. Returns an empty storage_linear in case of failure.
    ///
    /// @param Path Path of the file to open including filaname and filename extension
    texture load_kmg(std::string const& Path);

    /// Loads a texture storage_linear from KMG (Khronos Image) memory. Returns an empty storage_linear in case of failure.
    ///
    /// @param Data Pointer to the beginning of the texture container data to read
    /// @param Size Size of texture container Data to read
    texture load_kmg(char const* Data, std::size_t Size);

    /// Loads a texture storage_linear from KTX file. Returns an empty storage_linear in case of failure.
    ///
    /// @param Path Path of the file to open including filaname and filename extension
    texture load_ktx(char const* Path);

    /// Loads a texture storage_linear from KTX file. Returns an empty storage_linear in case of failure.
    ///
    /// @param Path Path of the file to open including filaname and filename extension
    texture load_ktx(std::string const& Path);

    /// Loads a texture storage_linear from KTX memory. Returns an empty storage_linear in case of failure.
    ///
    /// @param Data Pointer to the beginning of the texture container data to read
    /// @param Size Size of texture container Data to read
    texture load_ktx(char const* Data, std::size_t Size);
}

extern "C" {

    namespace bindings {

        namespace Load {

            Texture::texture load_load(char const* Data, std::size_t Size) {
                return gli::load(Data, Size);
            }

            Texture::texture load_load_dds(char const* Data, std::size_t Size) {
                return gli::load_dds(Data, Size);
            }

            Texture::texture load_load_kmg(char const* Data, std::size_t Size) {
                return gli::load_kmg(Data, Size);
            }

            Texture::texture load_load_ktx(char const* Data, std::size_t Size) {
                return gli::load_ktx(Data, Size);
            }
        }
    }//namespace gli
}

#ifdef GLI_IMPLEMENTATION
#include "load.inl"
#include "load_dds.inl"
#include "load_kmg.inl"
#include "load_ktx.inl"
#endif
