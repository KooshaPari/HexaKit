// Package auth provides shared authentication utilities for token storage.
package auth

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
)

// BaseTokenStorage provides common token storage functionality shared across providers.
type BaseTokenStorage struct {
	// FilePath is the path where the token file is stored.
	FilePath string `json:"-"`

	// Type indicates the authentication provider type.
	Type string `json:"type"`

	// AccessToken is the OAuth2 access token.
	AccessToken string `json:"access_token"`

	// RefreshToken is used to obtain new access tokens.
	RefreshToken string `json:"refresh_token"`

	// IDToken is the JWT ID token containing user claims.
	IDToken string `json:"id_token,omitempty"`

	// LastRefresh is the timestamp of the last token refresh.
	LastRefresh string `json:"last_refresh,omitempty"`

	// Expire is the timestamp when the access token expires.
	Expire string `json:"expired,omitempty"`
}

// NewBaseTokenStorage creates a new BaseTokenStorage with the given file path.
func NewBaseTokenStorage(filePath string) *BaseTokenStorage {
	return &BaseTokenStorage{
		FilePath: filePath,
	}
}

// Save writes the token storage to its file path as JSON.
func (b *BaseTokenStorage) Save() error {
	if b.FilePath == "" {
		return fmt.Errorf("base token storage: file path is empty")
	}

	dir := filepath.Dir(b.FilePath)
	if err := os.MkdirAll(dir, 0700); err != nil {
		return fmt.Errorf("failed to create directory: %w", err)
	}

	f, err := os.Create(b.FilePath)
	if err != nil {
		return fmt.Errorf("failed to create token file: %w", err)
	}
	defer f.Close()

	if err := json.NewEncoder(f).Encode(b); err != nil {
		return fmt.Errorf("failed to write token to file: %w", err)
	}
	return nil
}
