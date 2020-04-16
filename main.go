package main

import (
	"flag"
	"fmt"
	"html/template"
	"net/http"
	"os"
	"path/filepath"

	"github.com/gorilla/mux"
	"github.com/iterum-provenance/cluster/pipeline"
	metav1 "k8s.io/apimachinery/pkg/apis/meta/v1"
	"k8s.io/client-go/kubernetes"
	"k8s.io/client-go/tools/clientcmd"
)

type Page struct {
	Title string
}

func IndexHandler(w http.ResponseWriter, req *http.Request) {
	t, _ := template.ParseFiles("./static/index.html")
	t.Execute(w, Page{"test"})
}

func main() {

	var kubeconfig *string
	if home := homeDir(); home != "" {
		kubeconfig = flag.String("kubeconfig", filepath.Join(home, ".kube", "config"), "(optional) absolute path to the kubeconfig file")
	} else {
		kubeconfig = flag.String("kubeconfig", "", "absolute path to the kubeconfig file")
	}
	flag.Parse()

	// use the current context in kubeconfig
	config, err := clientcmd.BuildConfigFromFlags("", *kubeconfig)
	if err != nil {
		panic(err.Error())
	}

	// create the clientset
	clientset, err := kubernetes.NewForConfig(config)
	if err != nil {
		panic(err.Error())
	}
	namespace := "default"
	jobsClient := clientset.BatchV1().Jobs(namespace)

	r := mux.NewRouter()
	r.HandleFunc("/", IndexHandler)
	r.HandleFunc("/create_pipeline/", pipeline.Create(jobsClient))
	r.HandleFunc("/get_pipelines/", pipeline.Get(jobsClient))
	http.Handle("/", r)
	http.ListenAndServe(":8080", nil)

	// pods, err := clientset.CoreV1().Pods(namespace).List(metav1.ListOptions{})
	// if err != nil {
	// 	panic(err.Error())
	// }
	// fmt.Printf("There are %d pods in the cluster\n", len(pods.Items))
	// for _, pod := range pods.Items {
	// 	fmt.Printf("Pod: %s\n", pod.GetName())
	// }

	// services, err := clientset.CoreV1().Services(namespace).List(metav1.ListOptions{})
	// if err != nil {
	// 	panic(err.Error())
	// }
	// fmt.Printf("There are %d services in the cluster\n", len(services.Items))
	// for _, pod := range services.Items {
	// 	fmt.Printf("Service: %s\n", pod.GetName())
	// }

	jobs, err := jobsClient.List(metav1.ListOptions{})
	if err != nil {
		panic(err.Error())
	}
	fmt.Printf("There are %d jobs in the cluster\n", len(jobs.Items))
	for _, job := range jobs.Items {
		fmt.Printf("Removing older job named: %q\n", job.GetName())
		err2 := jobsClient.Delete(job.GetName(), &metav1.DeleteOptions{})
		if err2 != nil {
			panic(err2.Error())
		}
		err3 := clientset.CoreV1().Pods(namespace).DeleteCollection(&metav1.DeleteOptions{}, metav1.ListOptions{LabelSelector: "job-name=" + job.GetName()})
		if err3 != nil {
			panic(err3.Error())
		}
	}
}

func homeDir() string {
	if h := os.Getenv("HOME"); h != "" {
		return h
	}
	return os.Getenv("USERPROFILE") // windows
}
