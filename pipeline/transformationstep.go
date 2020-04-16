package pipeline

type TransformationStep struct {
	Name  string `json:"name"`
	Image string `json:"image"`
}
