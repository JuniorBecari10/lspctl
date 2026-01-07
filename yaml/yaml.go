package yaml

import (
	"errors"
	"fmt"
	"iter"
	"maps"
	"strings"

	"gopkg.in/yaml.v3"
)

type ParsedYaml struct {
    Name string `yaml:"name"`
    Desc string `yaml:"description"`
    Homepage string `yaml:"homepage"`
    
    Licenses []string `yaml:"licenses"`
    Languages []string `yaml:"languages"`
    Categories []string `yaml:"categories"`

    Source rawSource `yaml:"source"`
    Bin map[string]string `yaml:"bin"`
}

type rawSource struct {
    Id string `yaml:"id"`
    // TODO: add more
}

// ---

type InstallYaml struct {
    Name string
    Source Source
    BinName string
}

type parsedPurl struct {
    RawSource string
    Name string
    Version string
}

type PackageSource int
const (
    SourceGo PackageSource = iota
    SourceNpm
    SourceGitHub
    SourceGeneric
    SourceUnknown
)

type Source struct {
    PkgSource PackageSource
    Name string
    Version string
}

// ---

func formatParsed(p *ParsedYaml) {
    p.Desc = strings.ReplaceAll(p.Desc, "\n", " ") // make the entire description a single line
    p.Desc = strings.TrimSpace(p.Desc) // prevent leading and trailing spaces
}

func parsePurl(id string) (parsedPurl, error) {
    const prefix = "pkg:"

    if !strings.HasPrefix(id, prefix) {
        return parsedPurl{}, fmt.Errorf("not a pkg PURL: '%s'", id)
    }

    rest := strings.TrimPrefix(id, prefix)
    parts := strings.SplitN(rest, "/", 2)
    if len(parts) != 2 {
        return parsedPurl{}, fmt.Errorf("Invalid PURL: '%s'", id)
    }

    source := parts[0]
    nameVersion := parts[1]

    name := nameVersion
    version := ""

    if at := strings.LastIndex(nameVersion, "@"); at != -1 {
        name = nameVersion[:at]
        version = nameVersion[at+1:]
    }

    return parsedPurl{
        RawSource: source,
        Name: name,
        Version: version,
    }, nil
}

func normalizeSource(raw string) (PackageSource, error) {
    switch raw {
        case "golang": return SourceGo, nil
        case "npm": return SourceNpm, nil
        case "github": return SourceGitHub, nil
        case "generic": return SourceGeneric, nil
        default: return SourceUnknown, fmt.Errorf("Unsupported source: '%s'", raw)
    }
}

func parseSource(purl parsedPurl) (Source, error) {
    normal, err := normalizeSource(purl.RawSource)
    if err != nil {
        return Source{}, err
    }

    return Source{
        PkgSource: normal,
        Name: purl.Name,
        Version: purl.Version,
    }, nil
}

// ---

func ParseYaml(file string) (ParsedYaml, error) {
 	var parsed ParsedYaml
	if err := yaml.Unmarshal([]byte(file), &parsed); err != nil {
	    return ParsedYaml{}, err
	}

    formatParsed(&parsed)   
    return parsed, nil
}

func ConvertInstall(parsed ParsedYaml) (InstallYaml, error) {
    keys := maps.Keys(parsed.Bin)
    next, stop := iter.Pull(keys)
    defer stop()

    binName, ok := next()
    if !ok {
        return InstallYaml{}, errors.New("Empty 'bin'")
    }

    purl, err := parsePurl(parsed.Source.Id)
    if err != nil {
        return InstallYaml{}, err
    }
    
    source, err := parseSource(purl)
    if err != nil {
        return InstallYaml{}, err
    }
    
    return InstallYaml{
        Name: parsed.Name,
        Source: source,
        BinName: binName,
    }, nil
}
