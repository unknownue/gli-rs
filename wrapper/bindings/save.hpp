/// @brief Include to save DDS, KTX or KMG textures to files or memory.
/// @file gli/save.hpp

#pragma once

namespace gli {

    /// Save a texture storage_linear to a DDS file.
    ///
    /// @param Texture Source texture to save
    /// @param Path Path for where to save the file. It must include the filaname and filename extension.
    /// This function ignores the filename extension in the path and save to DDS anyway but keep the requested filename extension.
    /// @return Returns false if the function fails to save the file.
    bool save_dds(texture const & Texture, char const* Path);

    /// Save a texture storage_linear to a DDS file.
    ///
    /// @param Texture Source texture to save
    /// @param Path Path for where to save the file. It must include the filaname and filename extension.
    /// This function ignores the filename extension in the path and save to DDS anyway but keep the requested filename extension.
    /// @return Returns false if the function fails to save the file.
    bool save_dds(texture const & Texture, std::string const & Path);

    /// Save a texture storage_linear to a DDS file.
    ///
    /// @param Texture Source texture to save
    /// @param Memory Storage for the DDS container. The function resizes the containers to fit the necessary storage_linear.
    /// @return Returns false if the function fails to save the file.
    bool save_dds(texture const & Texture, std::vector<char> & Memory);

    /// Save a texture storage_linear to a KMG (Khronos Image) file.
    ///
    /// @param Texture Source texture to save
    /// @param Path Path for where to save the file. It must include the filaname and filename extension.
    /// This function ignores the filename extension in the path and save to KMG anyway but keep the requested filename extension.
    /// @return Returns false if the function fails to save the file.
    bool save_kmg(texture const & Texture, char const * Path);

    /// Save a texture storage_linear to a KMG (Khronos Image) file.
    ///
    /// @param Texture Source texture to save
    /// @param Path Path for where to save the file. It must include the filaname and filename extension.
    /// This function ignores the filename extension in the path and save to KMG anyway but keep the requested filename extension.
    /// @return Returns false if the function fails to save the file.
    bool save_kmg(texture const & Texture, std::string const & Path);

    /// Save a texture storage_linear to a KMG (Khronos Image) file.
    ///
    /// @param Texture Source texture to save
    /// @param Memory Storage for the KMG container. The function resizes the containers to fit the necessary storage_linear.
    /// @return Returns false if the function fails to save the file.
    bool save_kmg(texture const & Texture, std::vector<char> & Memory);

    /// Save a texture storage_linear to a KTX file.
    ///
    /// @param Texture Source texture to save
    /// @param Path Path for where to save the file. It must include the filaname and filename extension.
    /// This function ignores the filename extension in the path and save to KTX anyway but keep the requested filename extension.
    /// @return Returns false if the function fails to save the file.
    bool save_ktx(texture const & Texture, char const * Path);

    /// Save a texture storage_linear to a KTX file.
    ///
    /// @param Texture Source texture to save
    /// @param Path Path for where to save the file. It must include the filaname and filename extension.
    /// This function ignores the filename extension in the path and save to KTX anyway but keep the requested filename extension.
    /// @return Returns false if the function fails to save the file.
    bool save_ktx(texture const & Texture, std::string const & Path);

    /// Save a texture storage_linear to a KTX file.
    ///
    /// @param Texture Source texture to save
    /// @param Memory Storage for the KTX container. The function resizes the containers to fit the necessary storage_linear.
    /// @return Returns false if the function fails to save the file.
    bool save_ktx(texture const & Texture, std::vector<char> & Memory);
}//namespace gli


extern "C" {

    namespace bindings {

        namespace Save {

            bool save_save_dds(Texture::texture const & Texture, char const* Path) {
                return gli::save_dds(Texture, Path);
            }

            bool save_save_kmg(Texture::texture const & Texture, char const * Path) {
                return gli::save_kmg(Texture, Path);
            }

            bool save_save_ktx(Texture::texture const & Texture, char const * Path) {
                return gli::save_ktx(Texture, Path);
            }
        }
    }
}

#ifdef GLI_IMPLEMENTATION
#include "save_dds.inl"
#include "save_kmg.inl"
#include "save_ktx.inl"
#endif
